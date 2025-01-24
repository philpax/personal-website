// Monitor server liveness using WebSocket with fallback polling
(async () => {
  let disconnected = false;
  let ws = null;
  let pollInterval = null;

  const startWebSocket = () => {
    ws = new WebSocket(`ws://${window.location.host}/__poll_for_liveness`);

    ws.onclose = () => {
      disconnected = true;
      ws = null;
      console.log(
        "Lost connection to development server, waiting for reconnect..."
      );
      // Start polling when websocket disconnects
      if (!pollInterval) {
        pollInterval = setInterval(checkLiveness, 500);
      }
    };
  };

  const checkLiveness = async () => {
    try {
      await fetch("/__poll_for_liveness");

      if (disconnected) {
        // Server is back up, reload the page
        window.location.reload();
      }

      // Clear polling and restart websocket
      if (pollInterval) {
        clearInterval(pollInterval);
        pollInterval = null;
      }

      if (!ws) {
        startWebSocket();
      }

      disconnected = false;
    } catch {
      disconnected = true;
    }
  };

  // Start with WebSocket connection
  startWebSocket();
})();
