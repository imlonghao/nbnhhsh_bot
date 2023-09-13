const TOKEN = '114514'
const API = 'https://lab.magiconch.com/api/nbnhhsh/'
const HELP = '能不能好好说话 Bot 使用说明\n' +
  '\n' +
  '1. 私聊直接发送你不明白的缩写或包含缩写的文本\n' +
  '2. 群聊回复别人信息，加上 `/nbnhhsh` \n' +
  '3. 群聊通过 `/nbnhhsh kimo` 查询 kimo\n' +
  '\n' +
  '添加词条方法：\n' +
  '\n' +
  '私聊或在群内发送： `/add kimo 恶心` 以添加词条\n' +
  '\n' +
  '上游地址： [https://github.com/itorr/nbnhhsh](https://github.com/itorr/nbnhhsh) \n' +
  '机器人地址： [https://github.com/imlonghao/nbnhhsh_bot](https://github.com/imlonghao/nbnhhsh_bot) '

addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
});

function hash(str) {
  var hash = 0, i, chr;
  if (str.length === 0) return hash;
  for (i = 0; i < str.length; i++) {
    chr = str.charCodeAt(i);
    hash = ((hash << 5) - hash) + chr;
    hash |= 0;
  }
  return hash;
}

async function submitTran(word, text) {
  return await fetch(`${API}translation/${word}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      'text': text
    })
  })
}

async function guess(text) {
  const word = text.match(/[a-z0-9]+/g)
  if (word === null) {
    return ': 找不到相关信息'
  }
  const resp = await fetch(`${API}guess`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      'text': word.join(',')
    })
  })
  const respjson = await resp.json()
  let x = ''
  respjson.forEach((word) => {
    if (word.trans) {
      x += `${word.name}: ${word.trans.join(', ')}\n`
    } else if (word.inputting) {
      x += `${word.name}: (?) ${word.inputting.join(', ')}\n`
    } else {
      x += `${word.name}: 找不到相关信息\n`
    }
  })
  if (x === '') {
    x = ': 找不到相关信息'
  }
  return x
}

async function handleRequest(request) {
  if (request.method !== "POST") {
    return new Response()
  }
  const bot = new Telegram(TOKEN)
  const update = await request.json()
  if (update.inline_query) {
    const inline = update.inline_query
    let result = []
    let g = await guess(inline.query)
    g = g.trim()
    g.split('\n').forEach((x) => {
      result.push({
        type: 'article',
        id: hash(x),
        title: x.split(': ')[0],
        description: x.split(': ')[1].substring(0, 30),
        input_message_content: {
          message_text: x
        }
      })
    })
    return await bot.answerInlineQuery(inline.id, result)
  }
  if (update.message) {
    const message = update.message
    if (message.text === undefined) {
      return new Response()
    }
    const command = message.text.split(' ')
    switch (command[0]) {
      case '/start':
      case '/start@nbnhhsh_bot':
      case '/help':
      case '/help@nbnhhsh_bot':
        return await bot.sendMessage(message.from.id, HELP, 'markdown', true, true, message.message_id)
      case '/add':
      case '/add@nbnhhsh_bot':
        if (command.length != 3) {
          return await bot.sendMessage(message.from.id, '添加词语用法： `/add 缩写 中文`', 'markdown', true, true, message.message_id)
        }
        const resp = await submitTran(command[1], command[2])
        return await bot.sendMessage(message.from.id, '添加成功，管理员审核后可见', 'html', true, true, message.message_id)
      case '/nbnhhsh':
      case '/nbnhhsh@nbnhhsh_bot':
        if (command.length === 1) {
          if (message.reply_to_message === undefined) {
            return await bot.sendMessage(message.from.id, '请回复一条消息或在命令后加上想查询的词语', 'html', true, true, message.message_id)
          }
          return await bot.sendMessage(message.from.id, await guess(message.reply_to_message.text), 'html', true, true, message.message_id)
        } else {
          return await bot.sendMessage(message.from.id, await guess(message.text.replace(/^\/nbnhhsh /, '')), 'html', true, true, message.message_id)
        }
      default:
        return await bot.sendMessage(message.from.id, await guess(message.text), 'html', true, true, message.message_id)
    }
  }
  return new Response()
}

class Telegram {
  constructor(token) {
    this.api = 'https://api.telegram.org/bot' + token;
  }
  async sendMessage(chat_id, text, parse_mode, disable_web_page_preview, disable_notification,
    reply_to_message_id, reply_markup) {
    await fetch(this.api + '/sendMessage', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        chat_id: chat_id,
        text: text,
        parse_mode: parse_mode,
        disable_web_page_preview: disable_web_page_preview,
        disable_notification: disable_notification,
        reply_to_message_id: reply_to_message_id,
        reply_markup: reply_markup
      })
    })
    return new Response()
  }
  async answerInlineQuery(inline_query_id, results) {
    await fetch(this.api + '/answerInlineQuery', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        inline_query_id: inline_query_id,
        results: results
      })
    })
    return new Response()
  }
}
