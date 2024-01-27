api_banks = [{
        "id": 1660,
        "name": "Merkezi Bank ",
        "image": "merkezi.jpeg",
        "type": "DTB",
        "rate": 12,
        "percent":12,
        "credit":`15000 - 20000`,
    },
    {
        "id": 1709,
        "name": "Halk Bank ",
        "image": "halk.jpeg",
        "type": "DTB",
        "rate": 12,
        "percent":12,
        "credit":`15000 - 20000`,
    },
    {
        "id": 253,
        "name": "TÜRKMENISTANYŇ DÖWLET DAŞARY YKDYSADY IŞ BANKY ",
        "image": "vnesh.jpeg",
        "type": "DTB",
        "rate": 12,
        "percent":12,
        "credit":`15000 - 20000`,
    },
    {
        "id": 1707,
        "name": "Rysgal Bank ",
        "image": "rysgal bank.jpeg",
        "type": "DTB",
        "rate": 12,
        "percent":12,
        "credit":`15000 - 20000`,
    },
    {
        "id": 1650,
        "name": "Türkmenistan bank",
        "image": "tm-bank.webp",
        "type": "DTB",
        "rate": 12,
        "percent":12,
        "credit":`15000 - 20000`,
    },
    {
        "id": 1630,
        "name": "Türkmen-Türk banky",
        "image": "tt-bank.jpeg",
        "type": "DTB",
        "rate": 12,
        "percent":12,
        "credit":`15000 - 20000`,
    },
];


var showedBankId = api_banks[0]['id'];



function updateBank() {
    let index = api_banks.findIndex(obj => obj.id === showedBankId);

    document.getElementById('bank').innerHTML = `<div class="row my-2 my-sm-3">
    <div class="col-md-12">
    <div class="border bg-white rounded-table">
    <div class="d-flex align-items-center justify-content-center">
    <a href="#"><img src="img/` + api_banks[index].image + `" alt="" class="img-fluid border rounded" id="` + api_banks[index].id + `"></div></a>
    
    <div class="p-3">
        <div class="h3">`+api_banks[index].name+`</div>
        <div class="d-flex justify-content-between font-weight-bold">
            <div>
                <i class="bi bi-star-fill text-warning"></i>
                <i class="bi bi-star-fill text-warning"></i>
                <i class="bi bi-star-fill text-warning"></i>
                <i class="bi bi-star-fill text-warning"></i>
                <i class="bi bi-star text-secondary"></i>
                `+api_banks[index].rate+`bahalandyrma
            </div>
        </div>
        <div class="h4 pt-2">`+api_banks[index].type+`</div>
        <div class="h6 text-secondary">Bank görünişi</div>
        <div class="h4 pt-2">`+api_banks[index].percent+`%</div>
        <div class="h6 text-secondary">Ýyllyk göterim</div>
        <div class="h4 pt-2">`+api_banks[index].credit+`tmt</div>
        <div class="h6 text-secondary">2 ýyl möhlet bilen</div>
    </div>`;

    var products = document.getElementById('banks');
    products.innerHTML = "";
    for (let i = 0; i < api_banks.length; i++) {
        if (api_banks[i].id !== showedBankId) {
            products.innerHTML += `<div class="row no-gutters my-2 my-sm-3 border rounded bg-white">
                <div class="col-12">
                    <a href="#" class="text-decoration-none d-flex align-items-center justify-content-center" onclick="showBank(` + api_banks[i].id + `)">
                        <img src="img/` + api_banks[i].image + `" alt="" class="img-fluid border rounded">
                    </a>
                </div>
                <div class="col-12 py-2 text-center">
                    <a href="#" class="h5 text-dark text-decoration-none" onclick="showBank(` + api_banks[i].id + `)">` + api_banks[i].name + `</a>
                </div>
            </div>`;
        }
    }
}

function showBank(id) {
    showedBankId = id;
    updateBank();
}

updateBank();

function showValues(id){
    document.getElementById("value") = id
    .scrollIntoView();
}


