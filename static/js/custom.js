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
    var led_freq = $("#led_freq").val();
    var led_cycles = $("#led_cycles").val();
    var fan_freq = $("#fan_freq").val();
    var fan_cycles = $("#fan_cycles").val();

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
        "led_freq": parseInt(led_freq),
        "led_cycles": led_cycles,
        "fan_freq": parseInt(fan_freq),
        "fan_cycles": fan_cycles,
    });

    var xhr = new XMLHttpRequest();
    xhr.open("POST", "/save", true);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.send(data);
};

function element_on(pin, colour, code){
    $.ajaxSetup({
        async: false
    });

    var data = JSON.stringify({
        "pin" : pin,
        "colour" : colour,
        "code" : code,
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
        "code" : "",
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
        element_on(parseInt(heaven_pin), heaven_colour, "111");
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
        element_on(parseInt(cloud_pin), cloud_colour, "011");
    } else {
        element_off(parseInt(cloud_pin));
    }
};

function sun_test(element) {
    $.ajaxSetup({
        async: false
    });

    var sun_colour = $("#sun_colour").val();

    if (element.checked) {
        element_on(0, sun_colour, "101");
    } else {
        element_off(0);
    }
};

function thunder_test(element) {
    $.ajaxSetup({
        async: false
    });

    var thunder_colour = $("#thunder_colour").val();

    if (element.checked) {
        element_on(0, thunder_colour, "001");
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
        element_on(parseInt(wind_pin), wind_colour, "110");
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
        element_on(parseInt(water_pin), water_colour, "010");
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
        element_on(parseInt(mountain_pin), mountain_colour, "100");
    } else {
        element_off(parseInt(mountain_pin));
    }
};

function earth_test(element) {
    $.ajaxSetup({
        async: false
    });

    if (element.checked) {
        element_on(0, "rgb(0, 0, 0)", "000");
    } else {
        element_off(0);
    }
};

function apply_pwm(){
    $.ajaxSetup({
        async: false
    });

    var led_freq = $("#led_freq").val();
    var led_cycles = $("#led_cycles").val();
    var fan_freq = $("#fan_freq").val();
    var fan_cycles = $("#fan_cycles").val();

    var data = JSON.stringify({
        "led_freq": parseInt(led_freq),
        "led_cycles": led_cycles,
        "fan_freq": parseInt(fan_freq),
        "fan_cycles": fan_cycles,
    });

    $.ajax({
        url: "/pwm",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
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

function send_mail(){
    $.ajaxSetup({
        async: false
    });

    $.getJSON('/testing/mail', function() {
    });
};
