const eventSource = new EventSource('/api/sse');

eventSource.onmessage = function(event) {
  const notification = document.createElement('div');
  notification.className = 'notification';
  notification.innerHTML = event.data;
  document.body.appendChild(notification);
  console.log('SSE notification: ', event.data);

  setTimeout(() => {
    notification.classList.add('notification-remove');

    notification.addEventListener('animationend', () => {
      notification.remove();
    });
  }, 5000);
}

eventSource.onerror = function(event) {
  console.error('notification error: ', event);
}
