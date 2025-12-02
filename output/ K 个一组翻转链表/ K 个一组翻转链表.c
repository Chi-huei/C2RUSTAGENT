/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

struct ListNode* reverseKGroup(struct ListNode* head, int k){
    struct ListNode* newChain,*start,*next_start=NULL,*next,*p,*target;
    int len = 1;//链表长度
    int loop;//需要反转的片段数量
    //当k为1时直接返回原链表
    if(k==1){
        return head;
    }
    //获取新的表头
    p = head;
    for(int i=0;i<k-1;i++){
        p = p->next;
    };
    newChain = p;
    
    //获取链表长度
    next = head->next;
    while(next!=NULL){
        len ++;
        next = next->next;
    };
    //获取要循环的次数
    loop = len/k;
    start = head;
    next = newChain;
    while(loop+1>0){
        if(loop>0){
            p = start;
            for(int i=0;i<k;i++){
                p = p ->next;
            };
            next_start = p;   
        }
        int n2 = k-1;
        if(loop>0){
            for(int n=k;n>0;n--){
                target = start;
                int index = n2;
                while(index>0){
                    target = target->next;//target
                    index--;
                };
                n2 --;
                if(next!=target){
                    next->next = target;
                }
                next = target;
            };
        }else{
            next->next = start;
        }
        start = next_start;
         loop--;
    };
    return newChain;
}