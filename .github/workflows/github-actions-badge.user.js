// ==UserScript==
// @name         add badge for action link
// @namespace    https://github.com/akameco
// @version      0.1
// @description  add badge for action link
// @author       akameco (https://github.com/akameco)
// @match        https://github.com/*/*/actions*
// @grant        none
// ==/UserScript==

const el = document.querySelector('#badge-markdown')
el.textContent = `[${el.textContent}](${location.href})`