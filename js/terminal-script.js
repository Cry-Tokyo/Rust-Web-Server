var inputContainer = document.getElementById("inputContainer");
var num = 0;
const output = new Map([
    ["sum","\nType help for list of all commands\ntype fetch to retreive my summary\ntype GuI for real site"],
    ["fetch","copy neofetch and give my links and info out"],
    ["gui","send user to front end site instead of tui"],
    ["help","clear -- clear the terminal screen\ngui -- send user to normalsite"],
    ["tic","fun game"],
    ["weeb","anime chatters "],
    ["tools","nmpa ping all that stuff"]
]);
function createinput(){
    var div = document.createElement("div");
    div.id = 'div'+num;
    var label = document.createElement("label");
    label.textContent = "[guest@machine ~]$";
    var input = document.createElement("input");
    input.type = "text";
    input.id = "input" + num;
    inputContainer.appendChild(div);
    div.appendChild(label);
    div.appendChild(input);
    var newinput = document.getElementById("input" + num);
    newinput.focus();
    newinput.addEventListener("keydown",Respond);
    num += 1;
}
function replaceinput(input){
    var label = document.createElement("label");
    label.innerText = input;
    var cont = document.getElementById("inputContainer");
    cont.appendChild(label);
}
function Respond(event) {
    if (event.keyCode === 13) {
        var oldinput = document.getElementById("input" + (num -1));
        oldinput.readOnly = true;
        oldinput.removeEventListener("keydown", Respond);
        if (output.has(oldinput.value) === true ) {
            replaceinput(output.get(oldinput.value));
            createinput();
        } else if (oldinput.value === "clear"){
            location.reload();
        } else {
            createinput();
        }
    }
}
createinput();
