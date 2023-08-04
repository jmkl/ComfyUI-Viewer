(async function () {
  document.addEventListener('DOMContentLoaded',()=>{
    const { emit,listen } = window.__TAURI__.event;
    const {invoke} = window.__TAURI__.tauri;
    const cuiViewerHtml = `
    <div id="comfyuiviewer-content">
    <select id="wf">
    </select>
    <button class='apply_workflow' >Apply</button>
    </div>
    `;
  
    const cuiViewerStyle = `
    #comfyuiviewer-content{
      position:absolute;
      display:block;
      top:20px;
      left:20px;
      width:100%;
    }

  
    select {
      font-family: "Helvetica", "Arial", sans-serif;
      -webkit-appearance: none;
         -moz-appearance: none;
              appearance: none;
      border: 0;
      outline: 0;
      height: 2.5em;
      min-width:16rem;
      padding: 0 4em 0 1em;
      background: url(https://upload.wikimedia.org/wikipedia/commons/9/9d/Caret_down_font_awesome_whitevariation.svg) no-repeat right 0.8em center/1.4em, linear-gradient(to left, rgba(255, 255, 255, 0.3) 3em, rgba(255, 255, 255, 0.2) 3em);
      color: white;
      border-radius: 0.25em;
      box-shadow: 0 0 1em 0 rgba(0, 0, 0, 0.2);
      cursor: pointer;
    }
    select option {
      color: inherit;
      background-color: #222;
      border-radius: 2px;
      padding: 5px;
    }
    select:focus {
      outline: none;
    }
    select::-ms-expand {
      display: none;
    }

.apply_workflow{
  font-family: "Helvetica", "Arial", sans-serif;
  display: inline-block;
  padding: 0.7em 2em;
  margin-bottom: 60px;
  -webkit-appearance: none;
  appearance: none;
  background-color: #ff0081;
  color: #fff;
  border-radius: 4px;
  border: none;
  cursor: pointer;
  position: relative;
  transition: transform ease-in 0.1s, box-shadow ease-in 0.25s;
  box-shadow: 0 2px 25px rgba(255, 0, 130, 0.5);
}

.apply_workflow:active{

  transform: scale(0.9);
  background-color: #e60074;
  box-shadow: 0 2px 25px rgba(255, 0, 130, 0.2);
}
    `;

    const cuiViewerDiv = document.createElement('div');
    cuiViewerDiv.className = "comfyuiviewer";
    cuiViewerDiv.innerHTML = cuiViewerHtml;
    document.body.appendChild(cuiViewerDiv);
    const cuiViewerStyleElement = document.createElement('style');
    cuiViewerStyleElement.innerText = cuiViewerStyle;
    document.head.appendChild(cuiViewerStyleElement);
    var wf_dropdown = document.getElementById("wf");
    document.querySelector(".apply_workflow").addEventListener("click",()=>{
      invoke("apply_workflow",{data:wf_dropdown.value});
    });

    const mypee = document.querySelector(".comfyuiviewer-p");
    invoke("load_workflow").then(result=>{
      
      result.forEach(element => {
        var opt = document.createElement("option");
        opt.value = element;
        opt.textContent = element;
        wf_dropdown.appendChild(opt);
      });
    })
    listen("msg",(e)=>{
      window.app.loadGraphData(JSON.parse(e.payload.message));
    });
  
})


 })();

