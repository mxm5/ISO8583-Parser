<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust Command Runner</title>
</head>
<body>

    <h1>Rust Command Runner</h1>

    <form method="post">
        <label for="rustArgument">Enter Rust Argument:</label>
        <input required value="<?php echo $_POST['rustArgument']?? '' ?>" 
        type="text" id="rustArgument" name="rustArgument" placeholder="e.g., 061148656C6C6F">
        <button type="submit">Run Command</button>
    </form>

    <?php
    if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    
        $rustArgument = $_POST['rustArgument'];
        $output = [];
 
        // Use shell_exec to run the Rust command with the validated argument
        $result = shell_exec("target\\release\\emv_parser.exe $rustArgument");
        
        if ($result === null) {
            echo '<p>Error executing Rust command.</p>';
        } else {
            echo '<h2>Command Output:</h2>';
            echo '<pre>' . htmlspecialchars($result) . '</pre>';
        }
    }
    ?>

</body>
</html>
