function init_month() {
    switch (page) {
        case 'apr':
            msg.textContent = "みなさん。4月です";
            back.onclick = () => {
            };
            next.onclick = () => {
                changePage('start');
            };
            break;
        case 'may':
            msg.textContent = "みなさん。5月です";
            back.onclick = () => {
            };
            next.onclick = () => {
                changePage('start');
            };
            break;
        case 'jun':
            msg.textContent = "みなさん。6月です";
            back.onclick = () => {
            };
            next.onclick = () => {
                changePage('start');
            };
            break;
        case 'jul':
            msg.textContent = "みなさん。7月です";
            back.onclick = () => {
            };
            next.onclick = () => {
                changePage('start');
            };
            break;
        case 'aug':
            msg.textContent = "みなさん。8月です";
            back.onclick = () => {
            };
            next.onclick = () => {
                changePage('start');
            };
            break;
        case 'sep':
            msg.textContent = "みなさん。9月です";
            back.onclick = () => {
            };
            next.onclick = () => {
                changePage('start');
            };
            break;
        case 'oct':
            msg.textContent = "みなさん。10月です";
            back.onclick = () => {
            };
            next.onclick = () => {
                changePage('start');
            };
            break;
        case 'nov':
            msg.textContent = "みなさん。11月です";
            back.onclick = () => {
            };
            next.onclick = () => {
                changePage('start');
            };
            break;
        case 'dec':
            msg.textContent = "みなさん。12月です";
            back.onclick = () => {
            };
            next.onclick = () => {
                changePage('start');
            };
            break;
        case 'jan':
            msg.textContent = "みなさん。1月です";
            back.onclick = () => {
            };
            next.onclick = () => {
                changePage('start');
            };
            break;
        case 'feb':
            msg.textContent = "みなさん。2月です";
            back.onclick = () => {
            };
            next.onclick = () => {
                changePage('start');
            };
            break;
        case 'mar':
            msg.textContent = "みなさん。3月です";
            back.onclick = () => {
            };
            next.onclick = () => {
                changePage('start');
            };
            break;
        default:
            return -1;
    }
    return 0;
}

function draw_month() {
    switch (page) {
        case 'apr':
        case 'may':
        case 'jun':
        case 'jul':
        case 'aug':
        case 'sep':
        case 'oct':
        case 'nov':
        case 'dec':
        case 'jan':
        case 'feb':
        case 'mar':
            map(player[0].x, player[0].y);
            break;
        default:
            return -1
    }
    return 0;
}
