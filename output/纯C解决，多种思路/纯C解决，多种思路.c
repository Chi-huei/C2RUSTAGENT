//直接法
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */


struct ListNode* reverseKGroup(struct ListNode* head, int k){
    struct ListNode *p=head,*q=head;
    struct ListNode *r,*pre,*t;
    pre=(struct ListNode*)malloc(sizeof(struct ListNode));
    pre->next=p;
    int i,count=0;

    while(p)
    {
        t=p;
        count++;
        i=1;
        while(i<k&&q)//到尾部
        {
            q=q->next;
            i++;
        }
        if(!q)
        return head;
        while(p!=q)
        {
            r=p->next;
            p->next=q->next;
            q->next=p;
            p=r;
        }
        if(count==1)//第一次翻转
        head=q;
        pre->next=q;//连接
        pre=t;//后移
        p=t->next;
        q=p;
    }
    return head;
}