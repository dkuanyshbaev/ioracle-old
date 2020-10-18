$(function () {
    $('#c1, #c2, #c3, #c4, #c5, #c6, #c7, #c8, #c9, #c10').colorpicker({
        useAlpha: false,
        format: "rgb"
    });
});

function save(){
    var default_colour = $("#default_colour").val();
    var resting_colour = $("#resting_colour").val();
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
    var multiply = $("#multiply").val();
    var bias = $("#bias").val();
    var threshold = $("#threshold").val();

    var data = JSON.stringify({
        "default_colour": default_colour,
        "resting_colour": resting_colour,
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

    if (element.checked) {
        colour_on("rgb(0, 0, 0)", "000");
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
