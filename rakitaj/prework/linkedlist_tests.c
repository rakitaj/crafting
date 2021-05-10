#include "linkedlist.c"

void append_tests() {
    LinkedList_t* ll_append = LinkedList_new();
    Node_t *node1 = Node_new(12);
    LinkedList_append(ll_append, node1);
    LinkedList_append(ll_append, Node_new(13));
    LinkedList_append(ll_append, Node_new(14));
    LinkedList_pp(ll_append);
    printf("Append done.\n");
}

void prepend_tests() {
    LinkedList_t* ll_prepend = LinkedList_new();
    LinkedList_prepend(ll_prepend, Node_new(20));
    LinkedList_prepend(ll_prepend, Node_new(19));
    LinkedList_prepend(ll_prepend, Node_new(18));
    LinkedList_pp(ll_prepend);
    printf("Prepend done.\n");
}

void print_null_char_tests() {
    char null_term = '\0';
    printf("Null char looks like %c\n", null_term);
    printf("Null chat as int %i\n", null_term);
    printf("Null string looks like, %s\n", null_term);
    if (null_term == 0) {
        printf("Equal to 0\n");
    } else {
        printf("not equal to 0\n");
    }
}

int main() {
    append_tests();
    prepend_tests();
    printf("Program exit\n");
}