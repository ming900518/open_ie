# Open IE - 在 Windows 10/11 中透過 HTTP API 開啓 Internet Explorer

> [!WARNING]  
> 使用 IE 被視為不安全的做法，請優先考慮使用 [Edge 相容模式](https://support.microsoft.com/zh-tw/microsoft-edge/microsoft-edge-中的-internet-explorer-模式-6604162f-e38a-48b2-acd2-682dbac6f0de)

## API

> [!NOTE]
> 本程式開啓後並不會出現任何終端機/命令提示字元視窗，如要關閉本程式，可以透過下方的「關閉本程式」 API 或直接 kill 掉 `open_ie.exe`

1. 開啓 IE 並導向指定網頁

    > `http://_IP_:7000/?path=_欲在 IE 開啓的網址_`

2. 關閉本程式

    > `http://_IP_:7000/close`
