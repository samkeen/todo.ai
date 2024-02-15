const { invoke } = window.__TAURI__.tauri;

let headlineInputEl;
let descInputEl;
let todoMsgEl;

async function persist_todo() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  todoMsgEl.textContent = await invoke("persist_todo", {
    headline: headlineInputEl.value, description: descInputEl.value});
}

window.addEventListener("DOMContentLoaded", () => {
  // Create selectors for all the inputs in the form in index.html
  headlineInputEl = document.querySelector("#headline-input");
  descInputEl = document.querySelector("#desc-input");

  todoMsgEl = document.querySelector("#todo-msg");
  document.querySelector("#todo-form").addEventListener("submit", (e) => {
    e.preventDefault();
    persist_todo().then(r => console.log(r));
  });
});
