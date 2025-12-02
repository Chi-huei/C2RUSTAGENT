/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */


struct ListNode* reverseKGroup(struct ListNode* head, int k){
    struct ListNode *headp = head, *temp, *head0;
    head0 = head;
    int i;
    for(temp = headp, i = 0; temp && i < k; i++, temp = temp->next)
    ;                               //作用是确定出一个完整的反转小组
    if(!temp && i < k)
        return headp;               //若不足k元,直接返回原本头节点
    struct ListNode *cur, *newhead = temp;
    while(headp != temp)
    {
        cur = headp;
        headp = headp->next;
        cur->next = newhead;
        newhead = cur;
    }                               //“伪造”的递归核心,由迭代实现每个小组的反转
    head0->next = reverseKGroup(temp, k);
    return newhead;                 //返回该小组的“头节点”作为上一小组“尾节点”后继
}