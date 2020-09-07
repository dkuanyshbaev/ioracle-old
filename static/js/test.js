function on_off(element) {
    switch (element.id) {
        case "heaven":
            if (element.checked) {
                heaven_on(element);
            } else {
                heaven_off(element);
            }
        break;
        case "cloud":
            if (element.checked) {
                cloud_on(element);
            } else {
                cloud_off(element);
            }
        break;
        case "sun":
            if (element.checked) {
                sun_on(element);
            } else {
                sun_off(element);
            }
        break;
        case "wind":
            if (element.checked) {
                wind_on(element);
            } else {
                wind_off(element);
            }
        break;
        case "thunder":
            if (element.checked) {
                thunder_on(element);
            } else {
                thunder_off(element);
            }
        break;
        case "water":
            if (element.checked) {
                water_on(element);
            } else {
                water_off(element);
            }
        break;
        case "mountain":
            if (element.checked) {
                mountain_on(element);
            } else {
                mountain_off(element);
            }
        break;
        case "earth":
            if (element.checked) {
                earth_on(element);
            } else {
                earth_off(element);
            }
        break;
        default:
            console.log("unknown element");
    }
};

function heaven_on(e){
    $.ajaxSetup({
        async: false
    });

    var pin = $('#heaven_pin').val();
    var colour = $('#heaven_colour').val();

    var data = JSON.stringify({
        "pin" : parseInt(pin),
        "colour" : colour,
        "sound" : "",
    });

    $.ajax({
        url: "test/heaven/on",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function heaven_off(e){
    $.ajaxSetup({
        async: false
    });

    var pin = $('#heaven_pin').val();

    var data = JSON.stringify({
        "pin" : parseInt(pin),
        "colour" : "",
        "sound" : "",
    });

    $.ajax({
        url: "test/heaven/off",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function cloud_on(e){
    $.ajaxSetup({
        async: false
    });

    var pin = $('#cloud_pin').val();
    var colour = $('#cloud_colour').val();

    var data = JSON.stringify({
        "pin" : parseInt(pin),
        "colour" : colour,
        "sound" : "",
    });

    $.ajax({
        url: "test/cloud/on",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function cloud_off(e){
    $.ajaxSetup({
        async: false
    });

    var pin = $('#cloud_pin').val();

    var data = JSON.stringify({
        "pin" : parseInt(pin),
        "colour" : "",
        "sound" : "",
    });

    $.ajax({
        url: "test/cloud/off",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function sun_on(e){
    $.ajaxSetup({
        async: false
    });

    var pin = $('#sun_pin').val();
    var colour = $('#sun_colour').val();

    var data = JSON.stringify({
        "pin" : parseInt(pin),
        "colour" : colour,
        "sound" : "",
    });

    $.ajax({
        url: "test/sun/on",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function sun_off(e){
    $.ajaxSetup({
        async: false
    });

    var pin = $('#sun_pin').val();

    var data = JSON.stringify({
        "pin" : parseInt(pin),
        "colour" : "",
        "sound" : "",
    });

    $.ajax({
        url: "test/sun/off",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function wind_on(e){
    $.ajaxSetup({
        async: false
    });

    var pin = $('#wind_pin').val();
    var colour = $('#wind_colour').val();

    var data = JSON.stringify({
        "pin" : parseInt(pin),
        "colour" : colour,
        "sound" : "",
    });

    $.ajax({
        url: "test/wind/on",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function wind_off(e){
    $.ajaxSetup({
        async: false
    });

    var pin = $('#wind_pin').val();

    var data = JSON.stringify({
        "pin" : parseInt(pin),
        "colour" : "",
        "sound" : "",
    });

    $.ajax({
        url: "test/wind/off",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function thunder_on(e){
    $.ajaxSetup({
        async: false
    });

    var sound = $('#thunder_sound').val();
    var colour = $('#thunder_colour').val();

    var data = JSON.stringify({
        "pin" : 0,
        "colour" : colour,
        "sound" : sound,
    });

    $.ajax({
        url: "test/thunder/on",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function thunder_off(e){
    $.ajaxSetup({
        async: false
    });

    var sound = $('#thunder_sound').val();

    var data = JSON.stringify({
        "pin" : 0,
        "colour" : "",
        "sound" : sound,
    });

    $.ajax({
        url: "test/thunder/off",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function water_on(e){
    $.ajaxSetup({
        async: false
    });

    var pin = $('#water_pin').val();
    var colour = $('#water_colour').val();

    var data = JSON.stringify({
        "pin" : parseInt(pin),
        "colour" : colour,
        "sound" : "",
    });

    $.ajax({
        url: "test/water/on",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function water_off(e){
    $.ajaxSetup({
        async: false
    });

    var pin = $('#water_pin').val();

    var data = JSON.stringify({
        "pin" : parseInt(pin),
        "colour" : "",
        "sound" : "",
    });

    $.ajax({
        url: "test/water/off",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function mountain_on(e){
    $.ajaxSetup({
        async: false
    });

    var sound = $('#mountain_sound').val();
    var colour = $('#mountain_colour').val();

    var data = JSON.stringify({
        "pin" : 0,
        "colour" : colour,
        "sound" : sound,
    });

    $.ajax({
        url: "test/mountain/on",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function mountain_off(e){
    $.ajaxSetup({
        async: false
    });

    var sound = $('#mountain_sound').val();

    var data = JSON.stringify({
        "pin" : 0,
        "colour" : "",
        "sound" : sound,
    });

    $.ajax({
        url: "test/mountain/off",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function earth_on(e){
    $.ajaxSetup({
        async: false
    });

    var pin = $('#earth_pin').val();
    var colour = $('#earth_colour').val();

    var data = JSON.stringify({
        "pin" : parseInt(pin),
        "colour" : colour,
        "sound" : "",
    });

    $.ajax({
        url: "test/earth/on",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function earth_off(e){
    $.ajaxSetup({
        async: false
    });

    var pin = $('#earth_pin').val();

    var data = JSON.stringify({
        "pin" : parseInt(pin),
        "colour" : "",
        "sound" : "",
    });

    $.ajax({
        url: "test/earth/off",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

// function heaven_test(){
//     $.ajaxSetup({
//         async: false
//     });
//
//     var pin = $('#heaven_pin').val();
//     var colour = $('#heaven_colour').val();
//
//     var data = JSON.stringify({
//         "pin" : parseInt(pin),
//         "colour" : colour,
//         "sound" : "",
//     });
//
//     $.ajax({
//         url: "test/heaven",
//         type: "POST",
//         data: data,
//         contentType: "application/json; charset=utf-8",
//         dataType: "json",
//         success: function(){
//         }
//     });
// };
//
// function cloud_test(){
//     $.ajaxSetup({
//         async: false
//     });
//
//     var pin = $('#cloud_pin').val();
//     var colour = $('#cloud_colour').val();
//
//     var data = JSON.stringify({
//         "pin" : parseInt(pin),
//         "colour" : colour,
//         "sound" : "",
//     });
//
//     $.ajax({
//         url: "test/cloud",
//         type: "POST",
//         data: data,
//         contentType: "application/json; charset=utf-8",
//         dataType: "json",
//         success: function(){}
//     });
// };
//
// function sun_test(){
//     $.ajaxSetup({
//         async: false
//     });
//
//     var pin = $('#sun_pin').val();
//     var colour = $('#sun_colour').val();
//
//     var data = JSON.stringify({
//         "pin" : parseInt(pin),
//         "colour" : colour,
//         "sound" : "",
//     });
//
//     $.ajax({
//         url: "test/sun",
//         type: "POST",
//         data: data,
//         contentType: "application/json; charset=utf-8",
//         dataType: "json",
//         success: function(){}
//     });
// };
//
// function wind_test(){
//     $.ajaxSetup({
//         async: false
//     });
//
//     var pin = $('#wind_pin').val();
//     var colour = $('#wind_colour').val();
//
//     var data = JSON.stringify({
//         "pin" : parseInt(pin),
//         "colour" : colour,
//         "sound" : "",
//     });
//
//     $.ajax({
//         url: "test/wind",
//         type: "POST",
//         data: data,
//         contentType: "application/json; charset=utf-8",
//         dataType: "json",
//         success: function(){}
//     });
// };
//
// function thunder_test(){
//     $.ajaxSetup({
//         async: false
//     });
//
//     var sound = $('#thunder_sound').val();
//     var colour = $('#thunder_colour').val();
//
//     var data = JSON.stringify({
//         "pin" : 0,
//         "colour" : colour,
//         "sound" : sound,
//     });
//
//     $.ajax({
//         url: "test/thunder",
//         type: "POST",
//         data: data,
//         contentType: "application/json; charset=utf-8",
//         dataType: "json",
//         success: function(){}
//     });
// };
//
// function water_test(){
//     $.ajaxSetup({
//         async: false
//     });
//
//     var pin = $('#water_pin').val();
//     var colour = $('#water_colour').val();
//
//     var data = JSON.stringify({
//         "pin" : parseInt(pin),
//         "colour" : colour,
//         "sound" : "",
//     });
//
//     $.ajax({
//         url: "test/water",
//         type: "POST",
//         data: data,
//         contentType: "application/json; charset=utf-8",
//         dataType: "json",
//         success: function(){}
//     });
// };
//
// function mountain_test(){
//     $.ajaxSetup({
//         async: false
//     });
//
//     var sound = $('#mountain_sound').val();
//     var colour = $('#mountain_colour').val();
//
//     var data = JSON.stringify({
//         "pin" : 0,
//         "colour" : colour,
//         "sound" : sound,
//     });
//
//     $.ajax({
//         url: "test/mountain",
//         type: "POST",
//         data: data,
//         contentType: "application/json; charset=utf-8",
//         dataType: "json",
//         success: function(){}
//     });
// };
//
// function earth_test(){
//     $.ajaxSetup({
//         async: false
//     });
//
//     var pin = $('#earth_pin').val();
//     var colour = $('#earth_colour').val();
//
//     var data = JSON.stringify({
//         "pin" : parseInt(pin),
//         "colour" : colour,
//         "sound" : "",
//     });
//
//     $.ajax({
//         url: "test/earth",
//         type: "POST",
//         data: data,
//         contentType: "application/json; charset=utf-8",
//         dataType: "json",
//         success: function(){}
//     });
// };
