<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>EMV Messgae Parser</title>
</head>
<body>

    <h1>EMV Message Parser</h1>

    <form method="get">
        <label for="lengthCheckBox">Add lenght to the message:</label>
        <input type="checkbox" name="lengthCheckBox" id="lengthCheckBox"
        <?php if ( isset($_GET['lengthCheckBox']) && $_GET['lengthCheckBox'] == "on") {echo "checked";}?>>
        <br/>
        <label for="headerCheckbox">Add default header to the message:</label>
        <input type="checkbox" name="headerCheckbox" id="headerCheckbox"
        <?php if ( isset($_GET['headerCheckbox']) && $_GET['headerCheckbox'] == "on") {echo "checked";}?>>
        <br/>
        <br/>
        <label for="Message">Enter the message:</label>
        <br/>
        <span style="color:Gray">(e.g. '0012600008000001002000000000000000930000')</span>
        <br/>
        <textarea required value="<?php echo $_GET['emv_message']?? '' ?>"
        type="text" id="emv_message" name="emv_message"
        rows="10" cols="50" ><?php if ( isset($_GET['emv_message'])) echo $_GET['emv_message']?></textarea>
        <br/>
        <button type="submit">Parse Message</button>
    </form>

    <?php
    if (isset($_GET['emv_message'])) {
    
        $emv_message = $_GET['emv_message'];

        if(isset($_GET['headerCheckbox'])) {
            $emv_message = "UnkownHead".htmlspecialchars($emv_message);
        }
        if(isset($_GET['lengthCheckBox'])) {
            $length = strlen($emv_message);
            if($length%2 == 0) {$length = $length / 2;}
            else { echo '<font color="red">The message length is not correct</font>';}
            $hexLength = str_pad(dechex($length), 4, '0', STR_PAD_LEFT);
            $emv_message = $hexLength.htmlspecialchars($emv_message);
        }
        $output = [];

        // Use shell_exec to run the Rust command with the emv_message as argument
        exec("target\\release\\emv_parser.exe $emv_message 2>&1", $output, $returnCode);
        if ($returnCode !== 0) {
            echo '<p>Error parsing message. Return code: ' . $returnCode . '</p>';
            echo '<p>Error output:  </p>';
            echo '<font color="red">' . implode("<br/>", $output) . '</font>';
        } else {
            echo '<h2>Parsed Message:</h2>';
            echo '<pre>' . htmlspecialchars(implode("\n", $output)) . '</pre>';
        }
    }
    ?>

</body>
</html>
