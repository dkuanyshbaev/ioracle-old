function output(s){
    $('.output').append("<h4>" + s + "</h4>");
}

function run_simulation(){
    $.ajaxSetup({
        async: false
    });

    $('#run_button').prop('disabled', true);
    $('.output').empty();

    // touch 1
    var line1 = "";
    $.getJSON('/simulation/touch', function(line) {
        output("Line 1: " + line);
        line1 = line;
    });

    // touch 2
    var line2 = "";
    $.getJSON('/simulation/touch', function(line) {
        output("Line 2: " + line);
        line2 = line;
    });

    // touch 3
    var line3 = "";
    $.getJSON('/simulation/touch', function(line) {
        output("Line 3: " + line);
        line3 = line;
    });

    var top_lines = JSON.stringify({
        "first" : line1,
        "second" : line2,
        "third" : line3,
    });

    $.ajax({
        url: "simulation/element",
        type: "POST",
        data: top_lines,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(element){
            output("Element: " + element);
        }
    });

    // touch 4
    var line4 = "";
    $.getJSON('/simulation/touch', function(line) {
        output("Line 4: " + line);
        line4 = line;
    });

    // touch 5
    var line5 = "";
    $.getJSON('/simulation/touch', function(line) {
        output("Line 5: " + line);
        line5 = line;
    });

    // touch 6
    var line6 = "";
    $.getJSON('/simulation/touch', function(line) {
        output("Line 6: " + line);
        line6 = line;
    });

    var bottom_lines = JSON.stringify({
        "first" : line4,
        "second" : line5,
        "third" : line6,
    });

    $.ajax({
        url: "simulation/element",
        type: "POST",
        data: bottom_lines,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(element){
            output("Element: " + element);
        }
    });

    var all_lines = JSON.stringify({
        "first" : line1,
        "second" : line2,
        "third" : line3,
        "fourth" : line4,
        "fifth" : line5,
        "sixth" : line6,
    });

    $.ajax({
        url: "simulation/result",
        type: "POST",
        data: all_lines,
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        success: function(hexagram){
            output("Hexagram: " + hexagram);
        }
    });

    $('#run_button').prop('disabled', false);
}
