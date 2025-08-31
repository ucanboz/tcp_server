use gpiod::{Chip, Options};
use tokio::time::{sleep, Duration};

pub async fn gpio_blink_task() {
    let chip = match  Chip::new("gpiochip7") {
        Ok(chip) => chip,
        Err(e) => {
            eprint!("Warning: GPIO chip not avaialble: {e}");
            return;
        }
    };

    let opts = Options::output([5]) // configure lines offsets
        .values([false]);
    let output = match chip.request_lines(opts) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Warning: GPIO line not available: {e}");
            return; // skip blinking
        }
    };

    loop {
        if let Err(e) = output.set_values([true])
        {
            eprint!("Warning: Failed to set GPIO HIGH: {e}");
        };

        sleep(Duration::from_millis(500)).await;

        if let Err(e) = output.set_values([false])
        {
            eprint!("Warning: Failed to set GPIO LOW: {e}");
        };

        sleep(Duration::from_millis(500)).await;
    }

    // never reached, but just in case, set LOW on exit
    if let Err(e) = output.set_values([false])
        {
            eprint!("Warning: Failed to set GPIO LOW: {e}");
        };

}
