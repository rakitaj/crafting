#include <stdlib.h>

struct Node {
    int data;
    struct Node *next;
    struct Node *previous;
};

typedef struct Node Node_t;

struct LinkedList {
    Node_t *head;
    Node_t *tail;
};

typedef struct LinkedList LinkedList_t;

LinkedList_t* LinkedList_new() {
    LinkedList_t *linked_list = malloc(sizeof(LinkedList_t));
    return linked_list;
}

void LinkedList_add_to_front(LinkedList_t* linked_list) {
    
}