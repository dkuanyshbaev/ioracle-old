$(function () {
    $('#c1, #c2, #c3, #c4, #c5, #c6, #c7, #c8, #c9, #c10').colorpicker({
        useAlpha: false,
        format: "rgb"
    });
});

function save(){
    var heaven_pin = $("#heaven_pin").val();
    var heaven_colour = $("#heaven_colour").val();
    var cloud_pin = $("#cloud_pin").val();
    var cloud_colour = $("#cloud_colour").val();
    var sun_pin = $("#sun_pin").val();
    var sun_colour = $("#sun_colour").val();
    var wind_pin = $("#wind_pin").val();
    var wind_colour = $("#wind_colour").val();
    var thunder_colour = $("#thunder_colour").val();
    var water_pin = $("#water_pin").val();
    var water_colour = $("#water_colour").val();
    var mountain_pin = $("#mountain_pin").val();
    var mountain_colour = $("#mountain_colour").val();
    var earth_colour = $("#earth_colour").val();
    var default_colour = $("#default_colour").val();
    var resting_colour = $("#resting_colour").val();
    var multiply = $("#multiply").val();
    var bias = $("#bias").val();
    var threshold = $("#threshold").val();

    var data = JSON.stringify({
        "heaven_pin": parseInt(heaven_pin),
        "heaven_colour": heaven_colour,
        "cloud_pin": parseInt(cloud_pin),
        "cloud_colour": cloud_colour,
        "sun_pin": parseInt(sun_pin),
        "sun_colour": sun_colour,
        "wind_pin": parseInt(wind_pin),
        "wind_colour": wind_colour,
        "thunder_colour": thunder_colour,
        "water_pin": parseInt(water_pin),
        "water_colour": water_colour,
        "mountain_pin": parseInt(mountain_pin),
        "mountain_colour": mountain_colour,
        "earth_colour": earth_colour,
        "default_colour": default_colour,
        "resting_colour": resting_colour,
        "multiply": multiply,
        "bias": bias,
        "threshold": threshold,
    });

    console.log(data);

    var xhr = new XMLHttpRequest();
    xhr.open("POST", "/save", true);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.send(data);
};

function on_off(element, id) {
    if (element.checked) {
        pin_on(id);
    } else {
        pin_off(id);
    }
};

function pin_on(id){
    $.ajaxSetup({
        async: false
    });

    var pin = $('#pin' + id).val();
    var data = JSON.stringify({
        "pin" : parseInt(pin),
        "action" : 1,
    });

    $.ajax({
        url: "testing/pin",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function pin_off(id){
    $.ajaxSetup({
        async: false
    });

    var pin = $('#pin' + id).val();
    var data = JSON.stringify({
        "pin" : parseInt(pin),
        "action" : 0,
    });

    $.ajax({
        url: "testing/pin",
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
