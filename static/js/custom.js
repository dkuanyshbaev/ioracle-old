$(function () {
    $('#c1, #c2, #c3, #c4, #c5, #c6, #c7, #c8, #c9, #c10, #c11').colorpicker({
        useAlpha: false,
        format: "rgb"
    });
});

function save(){
    var default_colour = $("#default_colour").val();
    var resting_colour = $("#resting_colour").val();
    var li_colour = $("#li_colour").val();
    var heaven_pin = $("#heaven_pin").val();
    var heaven_colour = $("#heaven_colour").val();
    var cloud_pin = $("#cloud_pin").val();
    var cloud_colour = $("#cloud_colour").val();
    var sun_colour = $("#sun_colour").val();
    var thunder_colour = $("#thunder_colour").val();
    var wind_pin = $("#wind_pin").val();
    var wind_colour = $("#wind_colour").val();
    var water_pin = $("#water_pin").val();
    var water_colour = $("#water_colour").val();
    var mountain_pin = $("#mountain_pin").val();
    var mountain_colour = $("#mountain_colour").val();
    var earth_colour = $("#earth_colour").val();
    var reading = $("#reading").val();
    var multiply = $("#multiply").val();
    var bias = $("#bias").val();
    var threshold = $("#threshold").val();

    var data = JSON.stringify({
        "default_colour": default_colour,
        "resting_colour": resting_colour,
        "li_colour": li_colour,
        "heaven_pin": parseInt(heaven_pin),
        "heaven_colour": heaven_colour,
        "cloud_pin": parseInt(cloud_pin),
        "cloud_colour": cloud_colour,
        "sun_colour": sun_colour,
        "thunder_colour": thunder_colour,
        "wind_pin": parseInt(wind_pin),
        "wind_colour": wind_colour,
        "water_pin": parseInt(water_pin),
        "water_colour": water_colour,
        "mountain_pin": parseInt(mountain_pin),
        "mountain_colour": mountain_colour,
        "earth_colour": earth_colour,
        "reading": parseInt(reading),
        "multiply": multiply,
        "bias": bias,
        "threshold": threshold,
    });

    var xhr = new XMLHttpRequest();
    xhr.open("POST", "/save", true);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.send(data);
};

function heaven_colour(element) {
    $.ajaxSetup({
        async: false
    });

    var heaven_colour = $("#heaven_colour").val();

    if (element.checked) {
        colour_on(heaven_colour, "111");
    } else {
        colour_off();
    }
};

function heaven_pin(element) {
    $.ajaxSetup({
        async: false
    });

    var heaven_pin = $("#heaven_pin").val();

    if (element.checked) {
        pin_on(parseInt(heaven_pin));
    } else {
        pin_off(parseInt(heaven_pin));
    }
};

function cloud_colour(element) {
    $.ajaxSetup({
        async: false
    });

    var cloud_colour = $("#cloud_colour").val();

    if (element.checked) {
        colour_on(cloud_colour, "011");
    } else {
        colour_off();
    }
};

function cloud_pin(element) {
    $.ajaxSetup({
        async: false
    });

    var cloud_pin = $("#cloud_pin").val();

    if (element.checked) {
        pin_on(parseInt(cloud_pin));
    } else {
        pin_off(parseInt(cloud_pin));
    }
};

function sun_colour(element) {
    $.ajaxSetup({
        async: false
    });

    var sun_colour = $("#sun_colour").val();

    if (element.checked) {
        colour_on(sun_colour, "101");
    } else {
        colour_off();
    }
};

function thunder_colour(element) {
    $.ajaxSetup({
        async: false
    });

    var thunder_colour = $("#thunder_colour").val();

    if (element.checked) {
        colour_on(thunder_colour, "001");
    } else {
        colour_off();
    }
};

function wind_colour(element) {
    $.ajaxSetup({
        async: false
    });

    var wind_colour = $("#wind_colour").val();

    if (element.checked) {
        colour_on(wind_colour, "110");
    } else {
        colour_off();
    }
};

function wind_pin(element) {
    $.ajaxSetup({
        async: false
    });

    var wind_pin = $("#wind_pin").val();

    if (element.checked) {
        pin_on(parseInt(wind_pin));
    } else {
        pin_off(parseInt(wind_pin));
    }
};

function water_colour(element) {
    $.ajaxSetup({
        async: false
    });

    var water_colour = $("#water_colour").val();

    if (element.checked) {
        colour_on(water_colour, "010");
    } else {
        colour_off();
    }
};

function water_pin(element) {
    $.ajaxSetup({
        async: false
    });

    var water_pin = $("#water_pin").val();

    if (element.checked) {
        pin_on(parseInt(water_pin));
    } else {
        pin_off(parseInt(water_pin));
    }
};

function mountain_colour(element) {
    $.ajaxSetup({
        async: false
    });

    var mountain_colour = $("#mountain_colour").val();

    if (element.checked) {
        colour_on(mountain_colour, "100");
    } else {
        colour_off();
    }
};

function mountain_pin(element) {
    $.ajaxSetup({
        async: false
    });

    var mountain_pin = $("#mountain_pin").val();

    if (element.checked) {
        pin_on(parseInt(mountain_pin));
    } else {
        pin_off(parseInt(mountain_pin));
    }
};

function earth_colour(element) {
    $.ajaxSetup({
        async: false
    });

    var earth_colour = $("#earth_colour").val();

    if (element.checked) {
        colour_on(earth_colour, "000");
    } else {
        colour_off();
    }
};

function default_colour(element) {
    $.ajaxSetup({
        async: false
    });

    var default_colour = $("#default_colour").val();

    if (element.checked) {
        colour_on(default_colour, "111");
    } else {
        colour_off();
    }
};

function run_simulation(){
    $.ajaxSetup({
        async: false
    });

    $.getJSON('/operator/simulation', function() {
    });
};

function reset(){
    $.ajaxSetup({
        async: false
    });

    $.getJSON('/operator/reset', function() {
    });
};

function colour_on(colour, code){
    $.ajaxSetup({
        async: false
    });

    var data = JSON.stringify({
        "pin" : 0,
        "colour" : colour,
        "code" : code,
        "action" : 1,
    });

    $.ajax({
        url: "operator/colour",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function colour_off(){
    $.ajaxSetup({
        async: false
    });

    var data = JSON.stringify({
        "pin" : 0,
        "colour" : "",
        "code" : "",
        "action" : 0,
    });

    $.ajax({
        url: "operator/colour",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function pin_on(pin){
    $.ajaxSetup({
        async: false
    });

    var data = JSON.stringify({
        "pin" : pin,
        "colour" : "",
        "code" : "",
        "action" : 1,
    });

    $.ajax({
        url: "operator/pin",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function pin_off(pin){
    $.ajaxSetup({
        async: false
    });

    var data = JSON.stringify({
        "pin" : pin,
        "colour" : "",
        "code" : "",
        "action" : 0,
    });

    $.ajax({
        url: "operator/pin",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function play_sound(element, file_name){
    $.ajaxSetup({
        async: false
    });


    if (element.checked) {
        var data = JSON.stringify({
            "file_name" : file_name,
        });

        $.ajax({
            url: "operator/sound",
            type: "POST",
            data: data,
            contentType: "application/json; charset=utf-8",
            dataType: "json",
            success: function(){
            }
        });
    }
};

function li_test(element){
    $.ajaxSetup({
        async: false
    });

    if (element.checked) {
        colour_on("", "fire");
    }
};

function resting_colour(element) {
    $.ajaxSetup({
        async: false
    });

    var resting_colour = $("#resting_colour").val();
    var li_colour = $("#li_colour").val();

    if (element.checked) {
        resting_on(resting_colour, li_colour, "111");
    } else {
        resting_off();
    }
};

function li_colour(element) {
    $.ajaxSetup({
        async: false
    });

    var li_colour = $("#li_colour").val();
    var resting_colour = $("#resting_colour").val();

    if (element.checked) {
        li_on(li_colour, resting_colour);
    } else {
        li_off();
    }
};

function open_pip(){
    var multiply = $("#multiply").val();
    var bias = $("#bias").val();
    var threshold = $("#threshold").val();

    var data = JSON.stringify({
        "multiply": multiply,
        "bias": bias,
        "threshold": threshold,
    });

    var xhr = new XMLHttpRequest();
    xhr.open("POST", "operator/pip", true);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.send(data);
};

function show_emulation(){
    var first_trigram = $("#first_trigram").val();
    var second_trigram = $("#second_trigram").val();

    var li_colour = $("#li_colour").val();

    var data = JSON.stringify({
        "first_trigram": first_trigram,
        "second_trigram": second_trigram,
        "li_colour": li_colour,
    });

    var xhr = new XMLHttpRequest();
    xhr.open("POST", "/operator/emulation", true);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.send(data);
};

function li_on(li_colour, resting_colour){
    $.ajaxSetup({
        async: false
    });

    var data = JSON.stringify({
        "li_colour" : li_colour,
        "resting_colour" : resting_colour,
        "action" : 1,
    });

    $.ajax({
        url: "operator/li",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function li_off(){
    $.ajaxSetup({
        async: false
    });

    var data = JSON.stringify({
        "colour" : "",
        "action" : 0,
    });

    $.ajax({
        url: "operator/li",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function resting_on(yao_colour, li_colour, code){
    $.ajaxSetup({
        async: false
    });

    var data = JSON.stringify({
        "yao_colour" : yao_colour,
        "li_colour" : li_colour,
        "action" : 1,
    });

    $.ajax({
        url: "operator/resting",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function resting_off(){
    $.ajaxSetup({
        async: false
    });

    var data = JSON.stringify({
        "yao_colour" : "",
        "li_colour" : "",
        "action" : 0,
    });

    $.ajax({
        url: "operator/resting",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};
