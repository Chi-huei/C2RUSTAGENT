/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */


struct ListNode *reverseKGroup(struct ListNode* head, int k)
{
    struct ListNode *cur = head;
    int count = 0;
    // 找到待翻转的k个节点
    while(cur != NULL && count != k)
    {
        cur = cur ->next;
        count++;
    }
    if(count == k)
    {
        cur = reverseKGroup(cur,k);
        while(count != 0)
        {
            count--;
            struct ListNode *tmp = head ->next;
            head ->next = cur;
            cur = head;
            head = tmp;
        }
        head = cur;
    }
    //若剩余数量小于k的话，则不需要反转，因此直接返回待翻转部分的头结点即可
    return head;  //head为头指针
}