use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Buoy {
    pub time: String,
    pub model: String,
    pub lat: f64,
    pub lon: f64,
    pub w_temp: f32,
    pub salinity: f32,
    pub height: f32,
    pub weight: f32,
}

#[derive(Debug)]
pub struct Insertbuoy {
    pub buoy: Buoy,
    pub group_id: i32,
}

#[derive(Serialize, Debug)]
pub struct Modelinfo {
    pub model: String,
    pub group_id: i32,
    pub line: i32,
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Serialize, Debug)]
pub struct Group {
    pub group_id: i32,
    pub group_name: String,
    pub group_latitude: f32,
    pub group_longitude: f32,
    pub group_water_temp: f32,
    pub group_salinity: f32,
    pub group_height: f32,
    pub group_weight: f32,
}

#[derive(Serialize, Debug)]
pub struct GroupList {
    pub group_id: i32,
    pub group_name: String,
}

pub struct ObservatoryData {
    pub code: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

pub struct ObservatoryList {
    list: Vec<ObservatoryData>,
}

impl ObservatoryList {
    pub fn init() -> ObservatoryList {
        let mut list: Vec<ObservatoryData> = Vec::new();

        list.push(ObservatoryData {
            code: String::from("DT_0063"),
            name: String::from("가덕도"),
            lat: 35.024,
            lon: 128.81,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0032"),
            name: String::from("강화대교"),
            lat: 37.731,
            lon: 126.522,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0031"),
            name: String::from("거문도"),
            lat: 34.028,
            lon: 127.308,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0029"),
            name: String::from("거제도"),
            lat: 34.801,
            lon: 128.699,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0058"),
            name: String::from("경인항"),
            lat: 37.56,
            lon: 126.601,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0026"),
            name: String::from("고흥발포"),
            lat: 34.481,
            lon: 127.342,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0049"),
            name: String::from("광양"),
            lat: 34.903,
            lon: 127.754,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0042"),
            name: String::from("교본초"),
            lat: 34.704,
            lon: 128.306,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0018"),
            name: String::from("군산"),
            lat: 35.975,
            lon: 126.563,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0017"),
            name: String::from("대산"),
            lat: 37.007,
            lon: 126.352,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0065"),
            name: String::from("덕적도"),
            lat: 37.226,
            lon: 126.156,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0057"),
            name: String::from("동해항"),
            lat: 37.494,
            lon: 129.143,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0062"),
            name: String::from("마산"),
            lat: 35.197,
            lon: 128.576,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0023"),
            name: String::from("모슬포"),
            lat: 33.214,
            lon: 126.251,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0007"),
            name: String::from("목포"),
            lat: 34.779,
            lon: 126.375,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0006"),
            name: String::from("묵호"),
            lat: 37.55,
            lon: 129.116,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0025"),
            name: String::from("보령"),
            lat: 36.406,
            lon: 126.486,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0041"),
            name: String::from("복사초"),
            lat: 34.098,
            lon: 126.168,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0005"),
            name: String::from("부산"),
            lat: 35.096,
            lon: 129.035,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0056"),
            name: String::from("부산항신항"),
            lat: 35.077,
            lon: 128.786,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0061"),
            name: String::from("삼천포"),
            lat: 34.924,
            lon: 128.069,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0010"),
            name: String::from("서귀포"),
            lat: 33.24,
            lon: 126.561,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0051"),
            name: String::from("서천마량"),
            lat: 36.128,
            lon: 126.495,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0022"),
            name: String::from("성산포"),
            lat: 33.474,
            lon: 126.927,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0012"),
            name: String::from("속초"),
            lat: 38.207,
            lon: 128.594,
        });
        list.push(ObservatoryData {
            code: String::from("IE_0061"),
            name: String::from("신안가거초"),
            lat: 33.941,
            lon: 124.592,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0008"),
            name: String::from("안산"),
            lat: 37.192,
            lon: 126.647,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0067"),
            name: String::from("안흥"),
            lat: 36.674,
            lon: 126.129,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0037"),
            name: String::from("어청도"),
            lat: 36.117,
            lon: 125.984,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0016"),
            name: String::from("여수"),
            lat: 34.747,
            lon: 127.765,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0092"),
            name: String::from("여호항"),
            lat: 34.661,
            lon: 127.469,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0003"),
            name: String::from("영광"),
            lat: 35.426,
            lon: 126.42,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0044"),
            name: String::from("영종대교"),
            lat: 37.545,
            lon: 126.584,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0043"),
            name: String::from("영흥도"),
            lat: 37.238,
            lon: 126.428,
        });
        list.push(ObservatoryData {
            code: String::from("IE_0062"),
            name: String::from("옹진소청초"),
            lat: 37.423,
            lon: 124.738,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0027"),
            name: String::from("완도"),
            lat: 34.315,
            lon: 126.759,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0039"),
            name: String::from("왕돌초"),
            lat: 36.719,
            lon: 129.732,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0013"),
            name: String::from("울릉도"),
            lat: 37.491,
            lon: 130.913,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0020"),
            name: String::from("울산"),
            lat: 35.501,
            lon: 129.387,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0068"),
            name: String::from("위도"),
            lat: 35.618,
            lon: 126.301,
        });
        list.push(ObservatoryData {
            code: String::from("IE_0060"),
            name: String::from("이어도"),
            lat: 32.122,
            lon: 125.182,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0001"),
            name: String::from("인천"),
            lat: 37.451,
            lon: 126.592,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0052"),
            name: String::from("인천송도"),
            lat: 37.338,
            lon: 126.586,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0024"),
            name: String::from("장항"),
            lat: 36.006,
            lon: 126.687,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0004"),
            name: String::from("제주"),
            lat: 33.527,
            lon: 126.543,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0028"),
            name: String::from("진도"),
            lat: 34.377,
            lon: 126.308,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0021"),
            name: String::from("추자도"),
            lat: 33.961,
            lon: 126.3,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0050"),
            name: String::from("태안"),
            lat: 36.913,
            lon: 126.238,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0014"),
            name: String::from("통영"),
            lat: 34.827,
            lon: 128.434,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0002"),
            name: String::from("평택"),
            lat: 36.966,
            lon: 126.822,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0091"),
            name: String::from("포항"),
            lat: 36.047,
            lon: 129.383,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0066"),
            name: String::from("향화도"),
            lat: 35.167,
            lon: 126.359,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0011"),
            name: String::from("후포"),
            lat: 36.677,
            lon: 129.453,
        });
        list.push(ObservatoryData {
            code: String::from("DT_0035"),
            name: String::from("흑산도"),
            lat: 34.684,
            lon: 125.435,
        });


        ObservatoryList{ list }
    }
}

/*
DT_0063	가덕도	35.024	128.81
DT_0032	강화대교	37.731	126.522
DT_0031	거문도	34.028	127.308
DT_0029	거제도	34.801	128.699
DT_0058	경인항	37.56	126.601
DT_0026	고흥발포	34.481	127.342
DT_0049	광양	34.903	127.754
DT_0042	교본초	34.704	128.306
DT_0018	군산	35.975	126.563
DT_0017	대산	37.007	126.352
DT_0065	덕적도	37.226	126.156
DT_0057	동해항	37.494	129.143
DT_0062	마산	35.197	128.576
DT_0023	모슬포	33.214	126.251
DT_0007	목포	34.779	126.375
DT_0006	묵호	37.55	129.116
DT_0025	보령	36.406	126.486
DT_0041	복사초	34.098	126.168
DT_0005	부산	35.096	129.035
DT_0056	부산항신항	35.077	128.786
DT_0061	삼천포	34.924	128.069
DT_0010	서귀포	33.24	126.561
DT_0051	서천마량	36.128	126.495
DT_0022	성산포	33.474	126.927
DT_0012	속초	38.207	128.594
IE_0061	신안가거초	33.941	124.592
DT_0008	안산	37.192	126.647
DT_0067	안흥	36.674	126.129
DT_0037	어청도	36.117	125.984
DT_0016	여수	34.747	127.765
DT_0092	여호항	34.661	127.469
DT_0003	영광	35.426	126.42
DT_0044	영종대교	37.545	126.584
DT_0043	영흥도	37.238	126.428
IE_0062	옹진소청초	37.423	124.738
DT_0027	완도	34.315	126.759
DT_0039	왕돌초	36.719	129.732
DT_0013	울릉도	37.491	130.913
DT_0020	울산	35.501	129.387
DT_0068	위도	35.618	126.301
IE_0060	이어도	32.122	125.182
DT_0001	인천	37.451	126.592
DT_0052	인천송도	37.338	126.586
DT_0024	장항	36.006	126.687
DT_0004	제주	33.527	126.543
DT_0028	진도	34.377	126.308
DT_0021	추자도	33.961	126.3
DT_0050	태안	36.913	126.238
DT_0014	통영	34.827	128.434
DT_0002	평택	36.966	126.822
DT_0091	포항	36.047	129.383
DT_0066	향화도	35.167	126.359
DT_0011	후포	36.677	129.453
DT_0035	흑산도	34.684	125.435

*/

