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
    var sun_pin = $("#sun_pin").val();
    var sun_colour = $("#sun_colour").val();
    var thunder_colour = $("#thunder_colour").val();
    var wind_pin = $("#wind_pin").val();
    var wind_colour = $("#wind_colour").val();
    var water_pin = $("#water_pin").val();
    var water_colour = $("#water_colour").val();
    var mountain_pin = $("#mountain_pin").val();
    var mountain_colour = $("#mountain_colour").val();
    var earth_colour = $("#earth_colour").val();
    var multiply = $("#multiply").val();
    var bias = $("#bias").val();
    var threshold = $("#threshold").val();
    var led_pin = $("#led_pin").val();
    var led_freq = $("#led_freq").val();
    var led_cycles = $("#led_cycles").val();
    var fan_pin = $("#fan_pin").val();
    var fan_freq = $("#fan_freq").val();
    var fan_cycles = $("#fan_cycles").val();

    var data = JSON.stringify({
        "default_colour": default_colour,
        "resting_colour": resting_colour,
        "heaven_pin": parseInt(heaven_pin),
        "heaven_colour": heaven_colour,
        "cloud_pin": parseInt(cloud_pin),
        "cloud_colour": cloud_colour,
        "sun_pin": parseInt(sun_pin),
        "sun_colour": sun_colour,
        "thunder_colour": thunder_colour,
        "wind_pin": parseInt(wind_pin),
        "wind_colour": wind_colour,
        "water_pin": parseInt(water_pin),
        "water_colour": water_colour,
        "mountain_pin": parseInt(mountain_pin),
        "mountain_colour": mountain_colour,
        "earth_colour": earth_colour,
        "multiply": multiply,
        "bias": bias,
        "threshold": threshold,
        "led_pin": parseInt(led_pin),
        "led_freq": parseInt(led_freq),
        "led_cycles": led_cycles,
        "fan_pin": parseInt(fan_pin),
        "fan_freq": parseInt(fan_freq),
        "fan_cycles": fan_cycles,
    });

    var xhr = new XMLHttpRequest();
    xhr.open("POST", "/save", true);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.send(data);
};

function element_on(pin, colour){
    $.ajaxSetup({
        async: false
    });

    var data = JSON.stringify({
        "pin" : pin,
        "colour" : colour,
        "action" : 1,
    });

    $.ajax({
        url: "testing/element",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function element_off(pin){
    $.ajaxSetup({
        async: false
    });

    var data = JSON.stringify({
        "pin" : pin,
        "colour" : "",
        "action" : 0,
    });

    $.ajax({
        url: "testing/element",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function heaven_test(element) {
    $.ajaxSetup({
        async: false
    });

    var heaven_pin = $("#heaven_pin").val();
    var heaven_colour = $("#heaven_colour").val();

    if (element.checked) {
        element_on(parseInt(heaven_pin), heaven_colour);
    } else {
        element_off(parseInt(heaven_pin));
    }
};

function cloud_test(element) {
    $.ajaxSetup({
        async: false
    });

    var cloud_pin = $("#cloud_pin").val();
    var cloud_colour = $("#cloud_colour").val();

    if (element.checked) {
        element_on(parseInt(cloud_pin), cloud_colour);
    } else {
        element_off(parseInt(cloud_pin));
    }
};

function sun_test(element) {
    $.ajaxSetup({
        async: false
    });

    var sun_pin = $("#sun_pin").val();
    var sun_colour = $("#sun_colour").val();

    if (element.checked) {
        element_on(parseInt(sun_pin), sun_colour);
    } else {
        element_off(parseInt(sun_pin));
    }
};

function thunder_test(element) {
    $.ajaxSetup({
        async: false
    });

    var thunder_colour = $("#thunder_colour").val();

    if (element.checked) {
        element_on(0, thunder_colour);
    } else {
        element_off(0);
    }
};

function wind_test(element) {
    $.ajaxSetup({
        async: false
    });

    var wind_pin = $("#wind_pin").val();
    var wind_colour = $("#wind_colour").val();

    if (element.checked) {
        element_on(parseInt(wind_pin), wind_colour);
    } else {
        element_off(parseInt(wind_pin));
    }
};

function water_test(element) {
    $.ajaxSetup({
        async: false
    });

    var water_pin = $("#water_pin").val();
    var water_colour = $("#water_colour").val();

    if (element.checked) {
        element_on(parseInt(water_pin), water_colour);
    } else {
        element_off(parseInt(water_pin));
    }
};

function mountain_test(element) {
    $.ajaxSetup({
        async: false
    });

    var mountain_pin = $("#mountain_pin").val();
    var mountain_colour = $("#mountain_colour").val();

    if (element.checked) {
        element_on(parseInt(mountain_pin), mountain_colour);
    } else {
        element_off(parseInt(mountain_pin));
    }
};

function earth_test(element) {
    $.ajaxSetup({
        async: false
    });

    var earth_colour = $("#earth_colour").val();

    if (element.checked) {
        element_on(0, earth_colour);
    } else {
        element_off(0);
    }
};

function run_simulation(){
    $.ajaxSetup({
        async: false
    });

    $.getJSON('/testing/simulation', function() {
    });
};

function reset(){
    $.ajaxSetup({
        async: false
    });

    $.getJSON('/testing/reset', function() {
    });
};
