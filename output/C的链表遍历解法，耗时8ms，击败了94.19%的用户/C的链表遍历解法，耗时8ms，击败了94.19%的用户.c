/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

struct ListNode* reverse(struct ListNode* head, struct ListNode* tail) {
    struct ListNode* prev = NULL;
    struct ListNode* current = head;
    struct ListNode* next = head;

    if (NULL == head) {
        return NULL;
    }

    while (prev != tail && NULL != current) {
        next = current->next;
        current->next = prev;
        prev = current;
        current = next;
    }

    return tail;
} 


struct ListNode* reverseKGroup(struct ListNode* head, int k){

    struct ListNode* first = head;
    struct ListNode* last= head;
    struct ListNode* prev = NULL;
    struct ListNode* current = head;
    struct ListNode* next = head;
    struct ListNode* newHead = head;
    int i = 0;

    while (NULL !=  current) {
        i++;
        next = current->next;
        if (0 == i%k) {
            last = current;
            reverse(first,last);
            if (i == k) {
                newHead = last;
            }
            if (NULL != prev) {
                prev->next = last;
            }
            prev = first;
            first = next;
        }
        current = next;
    }

    if (0 != i%k && NULL != prev ) {
        prev->next = first;
    }

    return newHead;
}