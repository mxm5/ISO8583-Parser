<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>EMV Messgae Parser</title>
</head>
<body>

    <h1>EMV Message Parser</h1>

    <form method="post">
        <label for="lengthCheckBox">Add lenght to the message:</label>
        <input type="checkbox" name="lengthCheckBox" id="lengthCheckBox">
        <br/>
        <label for="headerCheckbox">Add default header to the message:</label>
        <input type="checkbox" name="headerCheckbox" id="headerCheckbox">
        <br/>
        <br/>
        <label for="Message">Enter the message:</label>
        <br/>

        <textarea required value="<?php echo $_POST['emv_message']?? '' ?>"
        type="text" id="emv_message" name="emv_message" placeholder="e.g., 061148656C6C6F"
        rows="10" cols="50" ></textarea>
        <br/>
        <button type="submit">Parse Message</button>
    </form>

    <?php
    if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    
        $emv_message = $_POST['emv_message'];

        if(isset($_POST['headerCheckbox'])) {
            $emv_message = "6000080000".htmlspecialchars($emv_message);
        }
        if(isset($_POST['lengthCheckBox'])) {
            $length = strlen($emv_message) /2;
            $hexLength = str_pad(dechex($length), 4, '0', STR_PAD_LEFT);
            $emv_message = $hexLength.htmlspecialchars($emv_message);
        }

        $output = [];
 
        // Use shell_exec to run the Rust command with the validated argument
        $result = shell_exec("target\\release\\emv_parser.exe $emv_message");
        
        if ($result === null) {
            echo '<p>Error parsing message.</p>';
        } else {
            echo '<h2>Parsed Message :</h2>';
            echo '<pre>' . htmlspecialchars($result) . '</pre>';
        }
    }
    ?>

</body>
</html>
