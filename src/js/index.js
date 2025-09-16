let username;
username = "$USERNAME";
const overlay = document.getElementById("overlay");
document.getElementById("username").innerText = username;
document.getElementById("folder-choose").addEventListener("click", blurfocus);
document.getElementById("file-choose").addEventListener("click", blurfocus);
function blurfocus() {
  overlay.style.display = "block";
}
