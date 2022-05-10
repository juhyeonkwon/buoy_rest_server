const axios = require('axios');

for (let i = 0; i < 1000; i++) {
  axios
    .get('http://192.168.0.20:3125/main/group/total', {
      headers: {
        accept: '*/*',
        Authorization: 'Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZHgiOjEsImVtYWlsIjoidGVzdEB0ZXN0LmNvbSIsImFkbWluIjoxLCJleHAiOjE5Njc0MzgyNjU2OTB9.Qu80hyH-pywivpboeR9oKMe_RVQz7PkbDvlwOa1Vw9I',
      },
    })
    .then((response) => {
      console.log(i);
    });
}

for (let i = 0; i < 1000; i++) {
  axios
    .get('http://192.168.0.20:3125/main/data?latitude=35.1513466&longitude=128.1001125', {
      headers: {
        accept: '*/*',
        Authorization: 'Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZHgiOjEsImVtYWlsIjoidGVzdEB0ZXN0LmNvbSIsImFkbWluIjoxLCJleHAiOjE5Njc0MzgyNjU2OTB9.Qu80hyH-pywivpboeR9oKMe_RVQz7PkbDvlwOa1Vw9I',
      },
    })
    .then((response) => {
      console.log(i);
    });
}
