const { invoke } = window.__TAURI__.tauri;

let headlineInputEl;
let descInputEl;
let todoMsgEl;

async function persist_todo() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    todoMsgEl.textContent = await invoke("persist_todo", {
        headline: headlineInputEl.value, description: descInputEl.value});
    // Update the list of todos
    get_todos();
}

async function update_todo(id, headline, description, is_done) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    todoMsgEl.textContent = await invoke("update_todo", {
        id: id, headline: headline, description: description, isDone: is_done}
    );
}

function get_todos() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    invoke("get_todos").then(todos => {
        const todoListEl = document.querySelector("#todo-list");
        todoListEl.innerHTML = "";
        todos.forEach(todo => {
            const todoHeadlineEl = document.createElement("dt");
            todoHeadlineEl.textContent = todo.headline;
            const todoIdEl = document.createElement("span");
            todoIdEl.textContent = todo.id;
            const todoDescriptionEl = document.createElement("dd");
            todoDescriptionEl.textContent = todo.description;
            const isDoneEl = document.createElement("input");
            isDoneEl.type = "checkbox";
            isDoneEl.checked = todo.is_done;
            // Add event listener to the checkbox
            isDoneEl.addEventListener('change', function() {
                update_todo(todo.id, todo.headline, todo.description, this.checked).then(r => console.log(r));
            });
            todoListEl.appendChild(todoHeadlineEl);
            todoListEl.appendChild(todoIdEl);
            todoListEl.appendChild(isDoneEl);
            todoListEl.appendChild(todoDescriptionEl);
        });
    });
}

window.onload = function() {
    console.log("onload");
  // Hide all sections
  document.querySelectorAll('main section').forEach(section => {
    section.style.display = 'none';
  });
  // Show the section-add initially
  document.getElementById('section-add').style.display = 'block';
  // Add event listeners to the navigation links
  document.querySelectorAll('nav a').forEach(link => {
    link.addEventListener('click', function(e) {
      e.preventDefault();

      // Hide all sections
      document.querySelectorAll('main section').forEach(section => {
        section.style.display = 'none';
      });

      // Show the section that corresponds to the clicked link
      const target = this.getAttribute('data-target');
      document.getElementById(target).style.display = 'block';
    });
  });
  // load the todos list
  get_todos();
};



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
