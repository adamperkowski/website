function setTargetBlank() {
  document.querySelectorAll('.target-blank').forEach(link => {
    link.setAttribute('target', '_blank');
  });
}

document.addEventListener('DOMContentLoaded', function () {
  setTargetBlank();
});
