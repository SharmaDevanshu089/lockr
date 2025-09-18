const invoke = window.__TAURI__.core.invoke;
const open = window.__TAURI__.dialog.open;
const overlay = document.getElementById("overlay");
console.log(open);

async function loadUser() {
  const username = await invoke("get_user");
  document.getElementById("username").innerText = username;
}

// Run it on page load
loadUser();

document.getElementById("folder-choose").addEventListener("click", blurfocus);
document.getElementById("file-choose").addEventListener("click", get_file);

function blurfocus() {
  overlay.style.display = "block";
}

async function get_file() {
  const file_to_open = await open({
    multiple: false,
    directory: false,
  });
  blurfocus();
  console.log(file_to_open);
}
