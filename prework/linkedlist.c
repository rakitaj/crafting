#include <stdlib.h>
#include <stdio.h>

struct Node {
    int data;
    struct Node *next;
    struct Node *previous;
};

typedef struct Node Node_t;

Node_t* Node_new(int value) {
    Node_t *node = malloc(sizeof(Node_t));
    node->data = value;
    node->next = NULL;
    node->previous = NULL;
    return node;
}

struct LinkedList {
    Node_t *head;
    Node_t *tail;
    int is_empty;
};

typedef struct LinkedList LinkedList_t;

LinkedList_t* LinkedList_new() {
    LinkedList_t *linked_list = malloc(sizeof(LinkedList_t));
    linked_list->is_empty = 1;
    return linked_list;
}

void LinkedList_append(LinkedList_t *linked_list, Node_t *node) {
    if (linked_list->is_empty == 1) {
        linked_list->head = node;
        linked_list->tail = node;
        linked_list->is_empty = 0;
    } else {
        Node_t *tail = linked_list->tail;
        tail->next = node;
        linked_list->tail = node;
    }
}

void LinkedList_prepend(LinkedList_t *ll, Node_t *node) {
    if (ll->is_empty == 1) {
        ll->head = node;
        ll->tail = node;
        ll->is_empty = 0;
    } else {
        Node_t *head = ll->head;
        ll->head = node;
        ll->head->next = head;
    }
}

void LinkedList_pp(LinkedList_t* linked_list) {
    Node_t* current = linked_list->head;
    while(current) {
        printf("%i\n", current->data);
        current = current->next;
    }
}
