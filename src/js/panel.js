(async function () {
  document.addEventListener("DOMContentLoaded", () => {




    if (location.href.includes("main.html")) return;
    function create_log_text(text) {
      var div = document.createElement("span");
      div.textContent = text;
      div.className = "log_text";
      return div;
    }

    const cuiViewerHtml = `
    
        <div class="titlebar">
        <div class="drag-region">(ComfyUI,)</div>
        <div class="titlebar-button-group">
            <div class="titlebar-button" onclick="window.ipc.postMessage('minimize')">
                <img src="https://api.iconify.design/codicon:chrome-minimize.svg" />
            </div>
            <div class="titlebar-button" onclick="window.ipc.postMessage('maximize')">
                <img src="https://api.iconify.design/codicon:chrome-maximize.svg" />
            </div>
            <div class="titlebar-button" id="close" onclick="window.ipc.postMessage('close')">
                <img src="https://api.iconify.design/codicon:close.svg" />
            </div>
        </div>
    </div>
    <main>
        <h4> WRYYYYYYYYYYYYYYYYYYYYYY! </h4>
    </main>
    <script>
        document.addEventListener('mousedown', (e) => {
            if (e.target.classList.contains('drag-region') && e.buttons === 1) {
                e.detail === 2
                    ? window.ipc.postMessage('maximize')
                    : window.ipc.postMessage('drag_window');
            }
        })
        document.addEventListener('touchstart', (e) => {
            if (e.target.classList.contains('drag-region')) {
                window.ipc.postMessage('drag_window');
            }
        })
    </script>
        <div id="comfyuiviewer-content">
          <button class='delete_wf workflowbtn' >Delete</button>
          <select id="wf">
          </select>
          <button class='apply_wf workflowbtn' >Apply</button>
          <button class='append_wf workflowbtn' >Append</button>
          <button class='save_wf workflowbtn' >Save Current Workflow</button>          
        </div>
        <div class="log">
        <span class="log_progress"></span>
        <div class="log_panel">
            <span class="log_text"></span>
          </div>
    </div>
        <div class="comfyuiviewer-modal">
     
        <div class="modal-content">      
          <input class="wf_filename" type="text" placeholder="workflows name"/>
          <button class='workflowbtn commit_wf' >Save</button>
          <button class='workflowbtn cancel_wf' >Cancel</button>
          </div>
        </div>
        `;

    const cuiViewerStyle = `
        body{
          background:#111;
        }
        .moveme{
          user-select:none;
          position:absolute;
          background-color:#ffdd00;
          padding:10px;
        }
        .titlebar {
          font-weight:900;
          position: absolute;
          top: 0px;
          left: 0px;
          width: 100%;
          z-index: 999999;
          display: flex;
          flex-flow:row wrap;        
          backdrop-filter: blur(30px);
          background: #00000042;
          color: white;
          user-select: none;
          box-shadow: 0 10px 10px #00000055;    
      
      
        }
        .drag-region{
          font-family: 'Roboto'!important;
          font-weight: 900;
          cursor:grab;
          flex-grow:1;
          padding:10px 20px
        }
        .titlebar-button-group{
          padding:5px 15px
        }

        .titlebar-button {
          cursor:pointer;
            display: inline-flex;
            justify-content: center;
            align-items: center;
            width: 30px;
            height: 30px;
        }

        .titlebar-button:hover {
            background: #3b3b3b;
        }

        .titlebar-button#close:hover {
            background: #da3d3d;
        }

        .titlebar-button img {
            filter: invert(100%);
        }
        #comfyuiviewer-content{
          
          
          position:absolute;
          display:flex;
          flex-direction:row;
          flex-wrap:nowrap;
          gap:5px;
          justify-content:center;
          align-items:center;
          top:80px;
          left:40px;
          
          
        }  
        #comfyuiviewer-content > *{
          
        }
        select#wf {
          
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
        select#wf option {
          color:inherit;
          background-color: #222;
          border-radius: 2px;
          padding: 5px;
        }
        select#wf:focus {
          outline: none;
        }
        select#wf::-ms-expand {
          display: none;
        }
    
      .workflowbtn{
        display: block;
        padding: 0.7em 2em;
        -webkit-appearance: none;
        appearance: none;
        background-color: #ff2124;
        color: #fff;
        border-radius: 4px;
        border: none;
        cursor: pointer;
        white-space: nowrap;
        box-shadow: 0 2px 25px #ff212430;
      }
    
      .workflowbtn:active{
        transform: scale(0.9);
        background-color: #ff2255;
        box-shadow: 0 2px 25px rgba(255, 0, 130, 0.2);
      }
      .comfyuiviewer-modal{
        display: none; 
        z-index: 1000;
        position:absolute;
        width:100%;
        height:100%;
        background:linear-gradient(#000,transparent);
        top:0;
        left:0;
    
      }
      .modal-content {
       
        gap:2px;
        display:flex;
        width:350px;
        margin:90px auto;
      }
      .modal-content > input[type="text"]{
        background:transparent;
        border:solid 1px rgba(255, 0, 130, 0.5);
        padding: 0.7em 2em;
        color:#fff;
        border-radius: 4px;
        box-shadow: 0 2px 25px rgba(255, 0, 130, 0.5);
        
        
      }
      .log{
        pointer-events:none;
        position:absolute;
        top:70px;
        left:10px;
        
    
      }
      .log_panel > span,
      .log > span{
        color:rgba(255,255,255,0.2);
        font-size:0.7rem;
        font-family:"CaskaydiaCove NF",consolas;
        white-space:nowrap;
        text-ofverflow:clip;
        overflow:hidden;
      }
      .log > span{
        color:#e60074;
      }
      
      #comfyuiviewer-content,
      .workflowbtn,
      select#wf,
      .modal-content > input[type="text"]
      {
        font-family: "Open Sans Condensed Medium";
      }
    
        `;

    const cuiViewerDiv = document.createElement("div");
    cuiViewerDiv.className = "comfyuiviewer";
    cuiViewerDiv.innerHTML = cuiViewerHtml;
    document.body.appendChild(cuiViewerDiv);
    const cuiViewerStyleElement = document.createElement("style");
    cuiViewerStyleElement.innerText = cuiViewerStyle;
    document.head.appendChild(cuiViewerStyleElement);
    var wf_dropdown = document.getElementById("wf");
    document.querySelector(".apply_wf").addEventListener("click", () => {

      const value = { key: "apply", data: { filename: wf_dropdown.value, data: "" } }
      window.ipc.postMessage(JSON.stringify(value));
    });


    var modal = document.querySelector(".comfyuiviewer-modal");
    var wf_filename = document.querySelector(".wf_filename");
    document.querySelector(".save_wf").addEventListener("click", () => {
      modal.style.display = "block";



    });
    document.querySelector(".delete_wf").addEventListener("click", () => {
      // invoke("delete_workflow", { filename: wf_dropdown.value }).then((result) => {
      //     wf_dropdown.remove(wf_dropdown.selectedIndex);
      // });
      const value = { key: "delete", data: { filename: wf_dropdown.value, data: "" } }
      console.log(wf_dropdown.value);
      window.ipc.postMessage(JSON.stringify(value))

    });

    document.querySelector(".cancel_wf").addEventListener("click", () => {
      modal.style.display = "none";

    });
    //hide for now
    document.querySelector(".append_wf").style.display = "none";
    document.querySelector(".append_wf").addEventListener("click", () => {

      const value = { key: "append", data: { filename: wf_dropdown.value, data: "" } }
      window.ipc.postMessage(JSON.stringify(value));
    });


    document.querySelector(".commit_wf").addEventListener("click", () => {

      var name = wf_filename.value;
      if (name.includes("json")) {
        name = name.replace(".json", "");
      }
      if (name === "")
        return;
      modal.style.display = "none";
      const json = JSON.stringify(app.graph.serialize(), null, 2);
      const value = { key: "save", data: { filename: name, data: json } }
      window.ipc.postMessage(JSON.stringify(value))

    });





    // invoke("load_workflow").then((result) => {
    //     result.forEach((element) => {
    //         var opt = document.createElement("option");
    //         opt.value = element;
    //         opt.textContent = element;
    //         wf_dropdown.appendChild(opt);
    //     });
    // });

    var logpanel = document.querySelector(".log_panel");
    var logprogress = document.querySelector(".log_progress");
    // listen("log_output", (e) => {

    //     const msg = e.payload.message;
    //     if (msg == "") return;


    //     while (logpanel.childNodes.length > 10) {
    //         logpanel.removeChild(logpanel.firstChild);
    //     }
    //     if (msg.includes("it\/s") || msg.includes("s\/it")) {
    //         logprogress.textContent = msg;
    //     } else {
    //         logpanel.append(create_log_text(`${e.payload.message}`));
    //         logpanel.append(document.createElement("br"));
    //     }




    // })
    // listen("msg", (e) => {
    //     window.app.loadGraphData(JSON.parse(e.payload.message));
    // });
  });

  document.addEventListener('mousedown', (e) => {
    if (e.target.classList.contains('drag-region') && e.buttons === 1) {
      e.detail === 2
        ? window.ipc.postMessage('maximize')
        : window.ipc.postMessage('drag_window');
    }
  })
  document.addEventListener('touchstart', (e) => {
    if (e.target.classList.contains('drag-region')) {
      window.ipc.postMessage('drag_window');
    }
  })

  let ismoving = true;
  const moveme_listener = function (e) {
    if (ismoving) {
      let move_me = document.querySelector('.moveme');
      let left = e.offsetX;
      let top = e.offsetY;
      move_me.style.left = (left - 5) + 'px';
      move_me.style.top = (top - 5) + 'px';
    }
  }


  setTimeout(() => {

    const value = { key: "load", data: { filename: "", data: "" } }
    window.ipc.postMessage(JSON.stringify(value));


  }, 500);



  // const result = document.addEventListener("mousemove", moveme_listener);
  // document.querySelector('.moveme').addEventListener("click", () => {
  //   document.removeEventListener("mousemove", moveme_listener)
  //   document.querySelector('.moveme').style.display = "none";
  // })
})();
