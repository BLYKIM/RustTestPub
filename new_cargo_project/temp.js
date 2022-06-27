//예제값
let 전체랜섬웨어 = 500;
let 전체피싱파밍 = 600;
let 전체악성코드 = 1000;
let 전체정보유출 = 1000;
let 랜섬웨어 = 200;
let 피싱파밍 = 400;
let 악성코드 = 1000;
let 정보유출 = 500;


//증감률 추가
let 이전랜섬웨어 = 300;
let 이전피싱파밍 = 200;
let 이전악성코드 = 2000;
let 이전정보유출 = 400;

//비율
let 랜섬비율 = ((랜섬웨어 - 이전랜섬웨어) / 이전랜섬웨어) * 100;
let 피싱비율 = ((피싱파밍 - 이전피싱파밍) / 이전피싱파밍) * 100;
let 악성비율 = ((악성코드 - 이전악성코드) / 이전악성코드) * 100;
let 정보비율 = ((정보유출 - 이전정보유출) / 이전정보유출) * 100;




option = {
    label: {
        formatter: [
            '{a|{b}}', '{b|{d}%}'].join('\n'),
        rich: {
            a: {
                fontSize: 16
            },
            b: {
                color: 'white',
                backgroundColor: 'black',
                padding: [3, 5, 1, 5],
                fontSize: 16,
                lineHeight: 20
            }
        }
    },
    color: ['#53B1AB', '#DEDEDE', '#53B1AB', '#53B1AB'], //컬러 앞 두가지값만 넣었을 시 버그 발생으로 임시조치
    series: [
        {
            type: 'pie',
            radius: [80, 60],
            center: ['20%', '50%'],
            data: [{ value: 랜섬웨어, label: { show: true }, name: '랜섬웨어' }, 전체랜섬웨어 - 랜섬웨어, 랜섬비율],
            label: {
                show: false,
                position: 'center'
            }
        },
        {
            type: 'pie',
            radius: [80, 60],
            center: ['40%', '50%'],
            data: [{ value: 피싱파밍, label: { show: true }, name: '피싱/파밍' }, 전체피싱파밍 - 피싱파밍],
            label: {
                show: false,
                position: 'center'
            }
        },
        {
            type: 'pie',
            radius: [80, 60],
            center: ['60%', '50%'],
            data: [{ value: 악성코드, label: { show: true }, name: '악성코드' }, 전체악성코드 - 악성코드],
            label: {
                show: false,
                position: 'center'
            }
        },
        {
            type: 'pie',
            radius: [80, 60],
            center: ['80%', '50%'],
            data: [{ value: 정보유출, label: { show: true }, name: '정보유출' }, 전체정보유출 - 정보유출],
            label: {
                show: false,
                position: 'center'
            }
        }
    ]
};
