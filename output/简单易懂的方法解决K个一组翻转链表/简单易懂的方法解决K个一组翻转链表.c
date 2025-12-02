/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
 
struct ListNode* reverseKGroup(struct ListNode* head, int k) {
	 struct ListNode* pre, * p;
	 struct ListNode *rear, * curhead,*curp;
	 int i;

	 curhead = (struct ListNode*)malloc(sizeof(struct ListNode));
	 pre = p = head;
	 while (p)
	 {
		 for (i = 1; i < k; i++)
		 {
			 if (!p)
				 break;
			 p = p->next;
		 }
		 if (p)
		 {
			 rear = p->next;
			 curhead->next = NULL;
			 if (pre == head)
				 head = curhead;
			 while (pre != rear)
			 {
				 curp = pre;
				 pre = pre->next;
				 curp->next = curhead->next;
				 curhead->next = curp;
			 }
			 while (curhead->next)
			 {
				 curhead = curhead->next;
			 }
			 curhead->next = rear;
			 p = pre;
		 }

	 }
	 return head->next;
 }