const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});


// When the Save button is clicked
document.getElementById('saveButton').addEventListener('click', function() {
    const content = document.getElementById('textArea').value;
    const filename = "output";

    // Call the Rust function
    window.__TAURI__.invoke('save_text', { content, filename })
        .then(() => alert("Saved successfully!"))
        .catch(err => console.error("Save error:", err));
});

var btn = document.getElementById('btn')
    
btn.addEventListener('click', function(){
document.getElementById('demo').innerHTML = "Hello JavaScript";
});