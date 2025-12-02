struct ListNode* reverseKGroup(struct ListNode* head, int k){
    if(k == 1){
        return head;
    }
    struct ListNode *p = head,*q,*rear,*result,*r;//p是要逆转的节点,q是防止断链，头插法，head，rear是正在翻转的k个节点组成的链表的表头和表尾，r是上个翻转完的k个节点链表的表尾
    int n = 0;
    while(p){
        n++;
        p = p->next;
    }
    if(n/k == 0){
        return head;
    }else{
        p = head->next;        
        r = head;
        for(int i=0;i<k-1 && p;i++){
            q = p->next;
            p->next = head;
            head = p;
            p = q;
        }
        result = head;  //记录整个链表的表头，也就是翻转第一个k链表的表头
    }
    for(int i=1;i<n/k;i++){
        head = p;
        rear = head;
        p = p->next;
        for(int j=0;j<k-1 && p;j++){
            q = p->next;
            p->next = head;
            head = p;
            p = q;
        }
        r->next = head;
        r = rear;
    }
    r->next = p;
    return result;
}