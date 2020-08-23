$(document).ready(function () {
    bsCustomFileInput.init()
});

$(function () {
    $('#c1, #c2, #c3, #c4, #c5, #c6, #c7, #c8').colorpicker({
        useAlpha: false
    });
});

function save(){
    var file_name = $("#filename").val();
    if (file_name) {
        var heaven_pin = $("#heaven_pin").val();
        var heaven_colour = $("#heaven_colour").val();
        var cloud_pin = $("#cloud_pin").val();
        var cloud_colour = $("#cloud_colour").val();
        var sun_pin = $("#sun_pin").val();
        var sun_colour = $("#sun_colour").val();
        var wind_pin = $("#wind_pin").val();
        var wind_colour = $("#wind_colour").val();
        var thunder_sound = $("#thunder_sound").val();
        var thunder_colour = $("#thunder_colour").val();
        var water_pin = $("#water_pin").val();
        var water_colour = $("#water_colour").val();
        var mountain_sound = $("#mountain_sound").val();
        var mountain_colour = $("#mountain_colour").val();
        var earth_pin = $("#earth_pin").val();
        var earth_colour = $("#earth_colour").val();
        var multiply = $("#multiply").val();
        var bias = $("#bias").val();
        var threshold = $("#threshold").val();

        var data = JSON.stringify({
            "file_name": file_name,
            "heaven_pin": parseInt(heaven_pin),
            "heaven_colour": heaven_colour,
            "cloud_pin": parseInt(cloud_pin),
            "cloud_colour": cloud_colour,
            "sun_pin": parseInt(sun_pin),
            "sun_colour": sun_colour,
            "wind_pin": parseInt(wind_pin),
            "wind_colour": wind_colour,
            "thunder_sound": thunder_sound,
            "thunder_colour": thunder_colour,
            "water_pin": parseInt(water_pin),
            "water_colour": water_colour,
            "mountain_sound": mountain_sound,
            "mountain_colour": mountain_colour,
            "earth_pin": parseInt(earth_pin),
            "earth_colour": earth_colour,
            "multiply": multiply,
            "bias": bias,
            "threshold": threshold,
        });

        var xhr = new XMLHttpRequest();
        xhr.open("POST", "/save", true);
        xhr.setRequestHeader("Content-Type", "application/json");
        xhr.send(data);

        $('#saveModal').modal('hide');
    }
};
