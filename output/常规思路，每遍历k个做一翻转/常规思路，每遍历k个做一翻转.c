/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
//输入为待翻转链表的前一个结点，输出仍为此结点，但链表已翻转
struct ListNode* reverse(struct ListNode* head) {
    struct ListNode *curfront,*cur,*curnext;
    if (head->next == NULL) {
        return NULL;
    }

    curfront = head->next;
    cur = curfront->next;
    curfront->next = NULL;

    while (cur) {
        curnext = cur->next;
        cur->next = curfront;
        curfront = cur;
        cur = curnext;
    }
    head->next = curfront;
    return head;
}
struct ListNode* reverseKGroup(struct ListNode* head, int k){
    struct ListNode* dummyhead = (struct ListNode*)malloc(sizeof(struct ListNode));
    struct ListNode *curfront = NULL,*curnext = NULL,*cur = NULL;
    int i = 1;
    if (head == NULL) return NULL;
    dummyhead->next = head;
    curfront = dummyhead;
    cur = dummyhead->next;

    while (cur) {
        if ((cur = cur->next) == NULL) break;
        if (++i == k) {
            curnext = cur->next;
            cur->next = NULL;//之前漏掉了断开链接这一步
            cur = curfront->next;//存下翻转后的尾
            reverse(curfront);
            cur->next = curnext;//断开链接之后还要链上
            curfront = cur;
            cur = curnext;
            i = 1;
        }
    }

    return dummyhead->next;

    
 }