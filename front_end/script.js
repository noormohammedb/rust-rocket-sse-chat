const eventSSE = new EventSource("/events");

const chatForm = document.querySelector("#chatForm");
const msgEle = document.getElementById("myMsg");

chatForm.addEventListener("submit", (e) => {
  e.preventDefault();
  const formData = Object.fromEntries(new FormData(chatForm).entries());
  //   console.log("formData: ", formData);
  axios({
    method: "post",
    url: "/msg",
    data: formData,
    headers: { "Content-Type": "multipart/form-data" },
  });

  msgEle.value = "";
  msgEle.focus();
});

const msgContainer = document.getElementById("msgWraper");
const msgCard = document.getElementById("msgCard");

const onMessageLogic = async (event) => {
  const msgCardClone = msgCard.cloneNode(true);

  const usernameEle = msgCardClone.getElementsByTagName("h5")[0];
  const roomEle = msgCardClone.getElementsByTagName("span")[0];
  const messageEle = msgCardClone.getElementsByTagName("p")[0];

  const eventData = JSON.parse(event.data);

  const { username, room, message } = eventData;

  usernameEle.innerText = username;
  messageEle.innerText = message;
  roomEle.innerText = room;

  msgCard.classList.remove("hidden");
  msgContainer.prepend(msgCardClone);
};

eventSSE.onmessage = onMessageLogic;
