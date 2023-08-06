mod api;
mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod core;

#[test]

fn test() {
    use tokio::runtime::Runtime;
    let url = "https://pipedproxy-bom-2.kavin.rocks/videoplayback?expire=1691340389&ei=BXrPZLzJOLTkjuMPuNuy4AE&ip=141.148.215.6&id=o-AKqQT8D0IreTJh3x9uqbXhpXOPQubIhe1Y7AqGhvmkK-&itag=140&source=youtube&requiressl=yes&mh=zn&mm=31%2C26&mn=sn-cvh7knzr%2Csn-h557sn6z&ms=au%2Conr&mv=m&mvi=5&pl=23&initcwndbps=3548750&spc=UWF9f7D8zeHrp4IA7laDNnTvAmeu7X4&vprv=1&svpuc=1&mime=audio%2Fmp4&gir=yes&clen=3525075&dur=217.733&lmt=1640353077691245&mt=1691318239&fvip=2&keepalive=yes&fexp=24007246&c=ANDROID&txp=5432434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cspc%2Cvprv%2Csvpuc%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIgeN1Zmg1F_QZ4NvMlBfIOlkLqw-wfqWhfi5pE2Yf1cdcCIQDcJ9w7vB6oZUYuumsJgD0Dx3pF5dW6sBJcKx5Up7wlaQ%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJQoSaDUifwhVBPjcQgCLavsC1j5cjKM81xvfhHyD9cYCIQD_4axGIlrbQfmIN863oaenRNxGtHSYG6kuHbFNrFh0sw%3D%3D&cpn=KFwdtnkscZp3Bmv9&host=rr5---sn-cvh7knzr.googlevideo.com";

    let mut client = core::client::HttpClient::new(None);

    let rt = Runtime::new().unwrap();

    let response = rt.block_on(async { client.get(url.to_string(), None).await });

    println!("Response {:?}", response);

    assert!(true);
}
