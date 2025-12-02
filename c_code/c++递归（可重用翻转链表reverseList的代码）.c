/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

struct ListNode* reverseList(struct ListNode* head) {
    if (head == NULL) return NULL;
    struct ListNode* cur = head;
    struct ListNode* prev = NULL;
    struct ListNode* next;
    while (cur) {
        next = cur->next;

        cur->next = prev;

        prev = cur;
        cur = next;
    }
    return prev;
}

struct ListNode* reverseKGroup(struct ListNode* head, int k) {
    if (head == NULL) return NULL;
    // [0, k-1] => [head, tail]
    struct ListNode* cur = head;
    for (int i = 0; i < k-1; ++i) {
        cur = cur->next;
        if (cur == NULL) return head;
    }
    struct ListNode* tail = cur;
    struct ListNode* nodek = cur->next;
    tail->next = NULL;
    struct ListNode* head1 = reverseList(head);
    struct ListNode* tail1 = head;
    struct ListNode* head2 = reverseKGroup(nodek, k);
    tail1->next = head2;
    return head1;
}