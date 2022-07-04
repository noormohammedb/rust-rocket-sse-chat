const eventSSE = new EventSource("/events");

const chatForm = document.getElementById("chatForm");
// console.log("chatForm: ", chatForm);

chatForm.addEventListener("submit", (e) => {
  e.preventDefault();
  console.log("submit");
});

const msgContainer = document.getElementById("msgWraper");
const msgCard = document.getElementById("msgCard");

const usernameEle = document.getElementById("username");
const roomEle = document.getElementById("room");
const messageEle = document.getElementById("message");

const onMessageLogic = async (event) => {
  console.log("event: ", event.data);
  console.log("event.data: ", event.data);

  const eventData = JSON.parse(event.data);

  const { username, room, message } = eventData;
  console.log("message: ", message);
  console.log("room,: ", room);
  console.log("username: ", username);

  usernameEle.innerText = username;
  roomEle.innerText = room;
  messageEle.innerText = message;
};

eventSSE.onmessage = onMessageLogic;
