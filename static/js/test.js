function heaven_test(){
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
        url: "test/heaven",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){
        }
    });
};

function cloud_test(){
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
        url: "test/cloud",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){}
    });
};

function sun_test(){
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
        url: "test/sun",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){}
    });
};

function wind_test(){
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
        url: "test/wind",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){}
    });
};

function thunder_test(){
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
        url: "test/thunder",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){}
    });
};

function water_test(){
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
        url: "test/water",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){}
    });
};

function mountain_test(){
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
        url: "test/mountain",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){}
    });
};

function earth_test(){
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
        url: "test/earth",
        type: "POST",
        data: data,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(){}
    });
};
