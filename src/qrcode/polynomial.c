#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <strings.h>

#include "encoding.h"
#include "utils.h"
#include "polynomial.h"
#include "log_antilog.h"


Polynomial* polynomial_alloc(size_t degree, NotationMode mode) {
    Polynomial* poli = (Polynomial*) malloc(sizeof(Polynomial));
    poli->values = (int*) malloc(sizeof(int) * degree);
    poli->degree = degree;
    poli->added_degree = 0;
    poli->mode = mode;

    return poli;
}

Polynomial* polynomial_alloc_zeros(size_t degree, NotationMode mode) {
    Polynomial* poli = (Polynomial*) malloc(sizeof(Polynomial));
    poli->values = (int*) calloc(degree, sizeof(int));
    poli->degree = degree;
    poli->added_degree = 0;
    poli->mode = mode;

    return poli;
}

Polynomial* polynomial_alloc_values(size_t degree, int* values, NotationMode mode) {
    Polynomial* poli = polynomial_alloc(degree, mode);

    while(degree--) {
        poli->values[degree] = values[degree];
    }

    return poli;
}

void polynomial_free(Polynomial* self) {
    free(self->values);
    free(self);
}

Polynomial* polynomial_get_1M_generator() {
    return polynomial_alloc_values(11, (int[11]) {
        0, 251, 67, 46, 61, 118, 70, 64, 94, 32, 45
    }, ALPHA);
}

Polynomial* polynomal_mul_alpha(Polynomial* self, int value) {
    assert(self->mode == ALPHA);
    for(size_t i = 0; i < self->degree; i++) {
        self->values[i] = (self->values[i] + value) % 255;
    }

    return self;
}

Polynomial* polynomial_convert(Polynomial* self) {
    LogAntilogTable* table = lat_initialize_table();
    int* selected_table = self->mode == DECIMAL ? table->log : table->antilog; // : table->log;
    self->mode = (self->mode + 1) % 2;

    for(size_t i = 0; i < table->size; i++) {
        self->values[i] = selected_table[(self->values[i] + self->added_degree) % 255];
    }
    
    return self;
}

Polynomial* polynomial_convert2(Polynomial* self) {
    LogAntilogTable* table = lat_initialize_table();
    int* selected_table = self->mode == DECIMAL ? table->log : table->antilog; // : table->log;
    self->mode = (self->mode + 1) % 2;

    for(size_t i = 0; i < table->size; i++) {
        self->values[i] = selected_table[self->values[i]];
    }
    
    return self;
}

Polynomial* polynomial_pop(Polynomial* self) {
    // if(self->degree > min) {
    //     self->degree--;
    // }
    self->degree--;

    for(size_t i = 0; i < self->degree; i++) {
        self->values[i] = self->values[i + 1];
    }
    // self->degree = 0;

    return self;
}

Polynomial* polynomial_convert_new(Polynomial* self) {
    LogAntilogTable* table = lat_initialize_table();
    Polynomial* poly = polynomial_alloc(self->degree, (self->mode + 1) % 2);
    int* selected_table = self->mode == DECIMAL ? table->log : table->antilog; // : table->log;

    for(size_t i = 0; i < table->size; i++) {
        poly->values[i] = selected_table[(self->values[i] + self->added_degree) % 255];
    }
    
    return poly;
}
void polynomial_display(Polynomial* self) {
    printf("Poli - mode: %s, degree: %zu, added_degree: %zu\n", 
        ((char*[2]) {"DECIMAL", "ALPHA"})[self->mode],
        self->degree, self->added_degree
    );

    for(size_t i = 0; i < self->degree; i++) {
        printf("%d%c", self->values[i], (i == self->degree - 1) ? '\n' : ',');
    }
}

Polynomial* polynomial_create_from_info(Array* info) {
    assert(!(info->size % 8));
    Polynomial* self = polynomial_alloc(info->size / 8, DECIMAL);
    Array* arr = utils_alloc_array(8);
    arr->size = 8;

    for(size_t i = 0; i < info->size; i += 8) {
        memcpy(arr->values, info->values + i, 8);
        self->values[i / 8] = encoding_decode_binary_to_int(arr);
    }

    utils_free_array(arr);
    return self;
}

// Polynomial* polynomial_xor(Polynomial* self, Polynomial* other) {
//     assert(self->mode == DECIMAL && other->mode == DECIMAL);
//     for(size_t i = 0; i < self->degree; i++) {
//         printf("XOR %zu %d %d\n", i, self->values[i], i < other->degree ? other->values[i] : 0);
//         self->values[i] ^= i < other->degree ? other->values[i] : 0;
//     }
//
//     return self;
// }

Polynomial* polynomial_xor(Polynomial* self, Polynomial* other) {
    assert(self->mode == DECIMAL && other->mode == DECIMAL);
    printf("MM %zu %zu\n", self->degree, other->degree);
    for(size_t i = 0; i < (self->degree > other->degree ? self->degree : other->degree); i++) {
        printf("XOR %zu %d %d\n", i, (i >= self->degree ? 0 : self->values[i]), i < other->degree ? other->values[i] : 0);
        self->values[i] = 
            (i >= self->degree ? 0 : self->values[i]) ^
            (i < other->degree ? other->values[i] : 0);
    }

    return self;
}


// Note: in our case we do seem to mind the exponent of the lead term,
// we only have to do the operations on values of the same index
// Note: a step per codewords,i.e. for 1-M: 10
Polynomial* polynomial_devide(Polynomial* gen, Polynomial* poly) {
    printf("Degrees: %zu %zu\n", gen->degree, poly->degree);
    LogAntilogTable* table = lat_initialize_table();
    int leading_alpha = poly->mode == ALPHA ? poly->values[0] : table->log[poly->values[0]];
    Polynomial* last_xor = NULL;

    polynomal_mul_alpha(gen, leading_alpha);
    printf("%d GEN\n", leading_alpha);
    polynomial_display(gen);
    polynomial_convert2(gen);
    polynomial_display(gen);


    // if(poly->mode == DECIMAL) {
    // polynomial_convert2(poly);
    // }

    polynomial_xor(poly, gen);

    polynomial_display(poly);
    polynomial_pop(poly);
    polynomial_display(poly);

    Polynomial* gen2 = polynomial_get_1M_generator();
    int leading_alpha2 = poly->mode == ALPHA ? poly->values[0] : table->log[poly->values[0]];
    polynomal_mul_alpha(gen2, leading_alpha2);
    polynomial_display(gen2);
    polynomial_convert2(gen2);
    polynomial_display(gen2);

    polynomial_xor(poly, gen2);

    polynomial_display(poly);
    polynomial_pop(poly);
    polynomial_display(poly);



    return NULL;

}

Polynomial* polynomial_devide2(Polynomial* poly) {
    LogAntilogTable* table = lat_initialize_table();
    size_t initial_degree = 16;

    while(initial_degree--) {
        Polynomial* gen = polynomial_get_1M_generator();
        int leading_alpha = poly->mode == ALPHA ? poly->values[0] : table->log[poly->values[0]];

        polynomal_mul_alpha(gen, leading_alpha);
        polynomial_convert2(gen);
        polynomial_xor(poly, gen);
        polynomial_pop(poly);
        // Note: allow the information polynomial to be XORed with the generator
        // that have a higher degree
        // TODO: in the `HELLO WORD` 1-M case, there is one degree of difference
        // that should change for other cases
        if(poly->degree < gen->degree) {
            poly->values[poly->degree++] = 0;
        }
        size_t tmp = poly->degree;
        poly->degree = 11;
        polynomial_display(poly);
        poly->degree = tmp;


        polynomial_free(gen);
    }

    printf("FINAL\n");
    // Note: reduce the degree by one because we kept it for the XOR operations
    // TODO: same as previous TODO, change for cases that are not `HELLOW WORLD` with 1-M
    poly->degree--;
    polynomial_display(poly);

    return poly;
}


// int main() {
//     Polynomial* generator = polynomial_get_1M_generator();
//     generator->added_degree = 5;
//
// }