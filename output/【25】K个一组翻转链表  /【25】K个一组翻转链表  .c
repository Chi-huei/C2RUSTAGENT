/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */


struct ListNode* reverse(struct ListNode *head,struct ListNode *tail)
{
    struct ListNode* pre = NULL;
    struct ListNode* next = NULL;
    while(head != tail)
    {
        next = head ->next;
        head ->next = pre;
        pre = head;
        head = next;
    }
    return pre;
}
struct ListNode*reverseKGroup(struct ListNode* head, int k)
{
    if(head == NULL || head ->next == NULL)
        return head;
    struct ListNode *newHead,*tail = head;
    int i;
    for(i = 0;i < k;i++)
    {
        //剩余数量小于k的话，不需要反转
        if(tail == NULL)
            return head;
        tail = tail ->next;
    }
    //反转前K个元素
    newHead = reverse(head,tail);
    //下一轮的开始的地方就是tail
    head ->next = reverseKGroup(tail,k);

    return newHead;
}