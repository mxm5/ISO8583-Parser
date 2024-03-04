<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ISO8583 Messgae Parser</title>
</head>
<body>

    <h1>ISO8583 Message Parser</h1>

    <form method="get">
        <label for="inlcudeLengthHeader">The message includes length and header</label>
        <input type="checkbox" name="inlcudeLengthHeader" id="inlcudeLengthHeader"
        <?php if ( isset($_GET['inlcudeLengthHeader']) && $_GET['inlcudeLengthHeader'] == "on") {echo "checked";}?>>
        <br/>
        <label for="parsePrivateTlv">Parse Private Tlv</label>
        <input type="checkbox" name="parsePrivateTlv" id="parsePrivateTlv"
        <?php if ( isset($_GET['parsePrivateTlv']) && $_GET['parsePrivateTlv'] == "on") {echo "checked";}?>>
        <br/>
        <label for="parsePrivateLtv">Parse Private Ltv</label>
        <input type="checkbox" name="parsePrivateLtv" id="parsePrivateLtv"
        <?php if ( isset($_GET['parsePrivateLtv']) && $_GET['parsePrivateLtv'] == "on") {echo "checked";}?>>
        <br/>
        <label for="Message">Enter the message:</label>
        <br/>
        <span style="color:Gray">(e.g. '01002000000000000000930000')</span>
        <br/>
        <span style="color:Gray">(e.g. '0012600008000001002000000000000000930000' 
        while including header and length)</span>
        <br/>
        <textarea required value="<?php echo $_GET['iso_message']?? '' ?>"
        type="text" id="iso_message" name="iso_message"
        rows="10" cols="50" ><?php if ( isset($_GET['iso_message'])) echo $_GET['iso_message']?></textarea>
        <br/>
        <button type="submit">Parse Message</button>
    </form>

    <?php
    if (isset($_GET['iso_message'])) {
    
        $iso_message = $_GET['iso_message'];
        $parser_arguments = "-m " . $iso_message;
        if(isset($_GET['inlcudeLengthHeader'])) {
            $parser_arguments = "-i " . $parser_arguments;
        }
        if(isset($_GET['parsePrivateTlv'])) {
            $parser_arguments = "-t " . $parser_arguments;
        }
        if(isset($_GET['parsePrivateLtv'])) {
            $parser_arguments = "-l " . $parser_arguments;
        }
        $output = [];

        // Use shell_exec to run the Rust command with the iso_message as argument
        exec("target\\release\\iso8583_parser.exe $parser_arguments 2>&1", $output, $returnCode);
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
