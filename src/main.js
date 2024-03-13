const { invoke } = window.__TAURI__.tauri;

/*
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


var btn = document.getElementById('btn')
    
btn.addEventListener('click', function(){
document.getElementById('demo').innerHTML = "Hello JavaScript";
});
*/
document.getElementById('js').addEventListener('click', hello);
function hello (){
  console.log("hello world!")
}
const divisadero = document.getElementById('divisadero').innerHTML;

async function saveText() {
  // Assuming 'textArea' is the ID of your textarea element
  const content = divisadero;
  // const content = document.getElementById('textArea').value;
  const filename = "output.txt";

  try {
    // Invoke the 'save_text' command from the backend (Rust side)
    // The 'save_text' Rust function needs to be defined accordingly and registered with Tauri
    await window.__TAURI__.invoke('savetext', { content, filename });

    // Handle successful save - Maybe notify the user or clear the textarea
    console.log("Text saved successfully!");
    // Optionally, clear the textarea or give feedback to the user
    // document.getElementById('textArea').value = "";
    //alert("Text was saved successfully!");
  } catch (err) {
    // Handle any errors that occur during the save process
    console.error("Failed to save text:", err);
    //alert(`Error saving text: ${err}`);
  }
}

// Example call to saveText(), could be tied to a button click event
document.getElementById('saveButton').addEventListener('click', saveText);

const callRustFunction = async () => {
  try {
      const response = await window.__TAURI__.invoke('my_simple_function');
      console.log(response); // Log the response from Rust
      document.getElementById('response').innerText = response;
  } catch (error) {
      console.error('Error calling Rust function: ', error);
  }
}

// Assuming you have a button to initiate the call
document.getElementById('call-rust-btn').addEventListener('click', callRustFunction);



