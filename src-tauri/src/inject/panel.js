(function () {
  document.addEventListener('DOMContentLoaded',()=>{
    const { emit,listen } = window.__TAURI__.event;
    const cuiViewerHtml = `
    <div id="comfyuiviewer-content">
    <p class="comfyuiviewer-p">Hello</p>
    </div>
    `;
  
    const cuiViewerStyle = `
    #comfyuiviewer-content{
      position:absolute;
      display:block;
      top:0;
      height:30px;
      width:100%;
      background:#000;
    }
   .comfyuiviewer-p {
      color:#fff;
      padding:5px;
      font-family:'Operator Mono Lig Light';
      word-break:break-all;
      white-space:nowrap;
      margin:0;

    }
    `;

    const cuiViewerDiv = document.createElement('div');
    cuiViewerDiv.className = "comfyuiviewer";
    cuiViewerDiv.innerHTML = cuiViewerHtml;
    document.body.appendChild(cuiViewerDiv);
    const cuiViewerStyleElement = document.createElement('style');
    cuiViewerStyleElement.innerText = cuiViewerStyle;
    document.head.appendChild(cuiViewerStyleElement);

    const mypee = document.querySelector(".comfyuiviewer-p");
    listen("msg",(e)=>{
      mypee.textContent = e.payload.message;
    });
  
})


 })();

