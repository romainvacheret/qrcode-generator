#include <stdbool.h>
#include <string.h>

#include "src/qrcode.h"
#include "src/gui.h"
#include "src/logger.h"

int main() {
    LOGGER = logger_alloc(DEBUG, stdout); 
    char* alph_string = "HELLO WORLD";
    EncodingMode encoding_mode = ALPHANUMERIC;
    ErrorCorrectionLevel correction_mode = M;
    // TODO: fix format string when mask != 2
    // Warning: do not use another mask!
    MaskPattern mask = MASK_2;
    /*
        FORMAT STRING with M
        masks:
            0: M/4 - 00/000 -> what to do? Should be: 101010000010010
            1: false, is: [1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0], should be: 101000100100101
            2: ok
            3: ok
            4: false, is: [1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0], should be 100010111111001
            5: false, is, [1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0], should be 100000011001110
            6: ok
            7: ok
    */
    int version = 1;
    QRCode* qrcode = qrcode_generate(
        alph_string,
        correction_mode,
        encoding_mode,
        mask,
        version
    );

    display_qrcode(qrcode);

    qrcode_free(qrcode);
    free(LOGGER);

    return EXIT_SUCCESS;
}
