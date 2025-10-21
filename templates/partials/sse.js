const eventSource = new EventSource('/api/sse');

function buildNotificationHTML(data) {
  const fireworkHTMLs = [
    '<img src="/img/gifs/fireworks-0.gif" alt="fireworks!" ',
    '<img src="/img/gifs/fireworks-1.gif" alt="fireworks!" ',
    '<img src="/img/gifs/fireworks-2.gif" alt="fireworks!" '
  ];

  let html = '<div class="notification-content"><p>' + data + '</p></div>';

  for (let i = 0; i < Math.floor(Math.random() * (25 - 10) ) + 10; i++) {
    for (const fireworkHTML of fireworkHTMLs) {
      if (Math.random() > 0.3) continue;
      html += fireworkHTML + 'style="position: fixed; top: ' + (Math.random() * 100) + 'vh; left: ' + (Math.random() * 100) + 'vw;" />';
    }
  }

  return html;
}

eventSource.onmessage = function(event) {
  const notification = document.createElement('div');
  notification.className = 'notification';
  notification.innerHTML = buildNotificationHTML(event.data);

  document.getElementById('content').appendChild(notification);

  console.log('got sse message');

  setTimeout(() => {
    const notificationContent = notification.querySelector('.notification-content');
    notificationContent.classList.add('notification-content-remove');
    notification.classList.add('notification-remove');

    notificationContent.addEventListener('animationend', () => {
      notification.remove();
    });
  }, 5000);
}

eventSource.onerror = function(event) {
  console.error('sse error: ', event);
}
