
const evtSource = new EventSource("/here");
evtSource.onmessage = (event) => {
      const dataTest = event.data;
}
