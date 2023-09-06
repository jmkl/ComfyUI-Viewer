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
        <div id="comfyuiviewer-content">
          <button class='delete_wf workflowbtn' >Delete</button>
          <select id="wf">
          </select>
          <button class='apply_wf workflowbtn' >Apply</button>
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
        #comfyuiviewer-content{
          
          
          position:absolute;
          display:flex;
          flex-direction:row;
          flex-wrap:nowrap;
          gap:5px;
          justify-content:center;
          align-items:center;
          top:20px;
          left:20px;
          
          
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
        background-color: #ff0081;
        color: #fff;
        border-radius: 4px;
        border: none;
        cursor: pointer;
        position: relative;
        transition: transform ease-in 0.1s, box-shadow ease-in 0.25s;
        box-shadow: 0 2px 25px rgba(255, 0, 130, 0.5);
      }
    
      .workflowbtn:active{
        transform: scale(0.9);
        background-color: #e60074;
        box-shadow: 0 2px 25px rgba(255, 0, 130, 0.2);
      }
      .comfyuiviewer-modal{
        display: none; 
        z-index: 100;
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
        margin:30px auto;
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
            // invoke("apply_workflow", { data: wf_dropdown.value });
            const value = { key: "apply", data: "applying me bitch" }
            window.ipc.postMessage(JSON.stringify(value))
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
            const value = { key: "delete", data: "delete me bitch" }
            window.ipc.postMessage(JSON.stringify(value))

        });

        document.querySelector(".cancel_wf").addEventListener("click", () => {
            modal.style.display = "none";

        });


        document.querySelector(".commit_wf").addEventListener("click", () => {

            var name = wf_filename.value;
            if (name.includes("json")) {
                name = name.replace(".json", "");
            }
            console.log(name);
            if (name === "")
                return;
            modal.style.display = "none";
            const json = JSON.stringify(app.graph.serialize(), null, 2);
            const value = { key: "save", data: "same me bitch" }
            window.ipc.postMessage(JSON.stringify(value))
            // invoke("save_workflow", { data: json, filename: name }).then((result) => {
            //     var opt = document.createElement("option");
            //     opt.value = result;
            //     console.log(result);
            //     opt.textContent = result;
            //     wf_dropdown.appendChild(opt);
            // });
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
})();
